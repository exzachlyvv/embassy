#![no_std]
#![no_main]

mod ov5640;

use embassy_executor::Spawner;
use embassy_stm32::dcmi::{self, *};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::khz;
use embassy_stm32::{bind_interrupts, i2c, peripherals, Config};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DCMI_PSSI => dcmi::InterruptHandler<peripherals::DCMI>;
});

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

static mut FRAME: [u32; WIDTH * HEIGHT / 2] = [0u32; WIDTH * HEIGHT / 2];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::{
            AHBPrescaler, APBPrescaler, ClockSrc, PllConfig, PllSource, Plldiv, Pllm, Plln, VoltageScale,
        };
        config.rcc.mux = ClockSrc::PLL1_R(PllConfig {
            source: PllSource::HSI, // 16 MHz
            m: Pllm::DIV4,          // 16 / 4 = 4
            n: Plln::MUL50,         // 4 * 50 = 200
            p: Plldiv::DIV2,        // 200 / 2 = 100
            q: Plldiv::DIV1,        // 200 / 1 = 200
            r: Plldiv::DIV2,        // 200 / 2 = 100
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_range = VoltageScale::RANGE1;
    }
    let peripherals = embassy_stm32::init(config);

    defmt::info!("Hello World!");

    // let mco = Mco::new(p.MCO, p.PA8, McoSource::HSI, McoPrescaler::DIV4);

    let cam_i2c = I2c::new(
        peripherals.I2C1,
        peripherals.PB8,
        peripherals.PB9,
        Irqs,
        peripherals.GPDMA1_CH1,
        peripherals.GPDMA1_CH2,
        khz(100),
        Default::default(),
    );

    // let mut buf: [u8; 1] = [0];
    // cam_i2c
    //     .blocking_write_read(0x3C, &[0x31, 0x00], &mut buf)
    //     .expect("I2C error:");

    // assert_eq!(&[0x78], &buf); // ID is 0x78.

    // defmt::info!("Read: {:?}", buf);

    let mut camera = ov5640::Ov5640::new(cam_i2c);

    camera
        .init(
            ov5640::Format::Raw(ov5640::RawOrder::SBGGR8),
            ov5640::Resolution::Qvga320_240,
        )
        .expect("Failed to initialize camera.");

    defmt::info!("CAMERA READY");

    let config = dcmi::Config::default();
    // original:
    // let mut dcmi = Dcmi::new_8bit(
    //     peripherals.DCMI, peripherals.DMA1_CH0, Irqs, peripherals.PC6, peripherals.PC7, peripherals.PE0, peripherals.PE1, peripherals.PE4, peripherals.PD3, peripherals.PE5, peripherals.PE6, peripherals.PB7, peripherals.PA4, peripherals.PA6, config,
    // );
    let mut dcmi = Dcmi::new_8bit(
        peripherals.DCMI,
        peripherals.GPDMA1_CH0,
        Irqs,
        peripherals.PC6,
        peripherals.PC7,
        peripherals.PC8,
        peripherals.PE1,
        peripherals.PH14,
        peripherals.PI4,
        peripherals.PI6,
        peripherals.PI7,
        peripherals.PB7,
        peripherals.PH8,
        peripherals.PA6,
        config,
    );

    loop {
        defmt::info!("Taking image...");

        let capture_result = dcmi.capture(unsafe { &mut FRAME }).await;

        match capture_result {
            Ok(()) => {
                defmt::info!("captured frame!!!!!!!!");

                for (index, pixel) in unsafe { FRAME }.iter().enumerate() {
                    if index < 10 {
                        defmt::info!("Data at ({}): {}", index, pixel);
                    }
                }
            }
            Err(err) => defmt::warn!("DCMI.capture error: {:#?}", err),
        };

        Timer::after_millis(2000).await;
    }
}
