#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_net;
use embassy_net_emw3080::Emw3080;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{ClockSrc, PllConfig, PllSource, Plldiv, Pllm, Plln};
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::khz;
use embassy_stm32::{peripherals, Config};
use embassy_time::{Delay, Timer};
use embedded_hal_bus::spi::ExclusiveDevice;
use {defmt_rtt as _, panic_probe as _};

const HTS221_ADDRESS: u8 = 0x5F;
const WHOAMI: u8 = 0x0F;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config: embassy_stm32::Config = Config::default();
    // config.rcc.mux = ClockSrc::PLL1_R(PllConfig {
    //     source: PllSource::HSI, // 16 MHz
    //     // The clock speed of the `source` divided by `m` must be between 4 and 16 MHz.
    //     // 16 MHz / 2 = 8 MHz
    //     m: Pllm::DIV2,
    //     // The multiplied clock – `source` divided by `m` times `n` – must be between 128 and 544
    //     // MHz. The upper limit may be lower depending on the `Config { voltage_range }`.
    //     // 8 MHz * 20 = 160 MHz
    //     n: Plln::MUL20,
    //     p: Plldiv::DIV1,
    //     q: Plldiv::DIV1,
    //     // When used to drive the system clock, `source` divided by `m` times `n` divided by `r`
    //     // must not exceed 160 MHz. System clocks above 55 MHz require a non-default
    //     // `Config { voltage_range }`.
    //     // 160 MHz / 3 = 53.33
    //     r: Plldiv::DIV3,
    // });

    let p = embassy_stm32::init(config);
    info!("running!");

    let eth_sck = p.PD1;
    let eth_mosi = p.PD4;
    let eth_miso = p.PD3;
    let eth_cs = p.PB12;
    let eth_rst = p.PF15;
    let _eth_irq = p.PD14;

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = khz(125);

    // TODO: use DMA to improve performance.
    let mut spi = Spi::new(p.SPI2, eth_sck, eth_mosi, eth_miso, NoDma, NoDma, spi_config);

    let cs = Output::new(eth_cs, Level::High, Speed::Medium);
    let spi = ExclusiveDevice::new(spi, cs, Delay);

    let rst = Output::new(eth_rst, Level::High, Speed::Medium);
    let mac_addr = [2, 3, 4, 5, 6, 7];
    let device = Emw3080::new(spi);

    let config = embassy_net::Config::dhcpv4(Default::default());

    info!("SPI setup completed.");

    loop {
        Timer::after_secs(1).await;

        info!("Done.");
    }
}
