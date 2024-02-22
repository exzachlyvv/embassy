#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use embedded_hal::spi::SpiDevice;

/// EMW3080 embassy-net driver
pub struct Emw3080<SPI> {
    spi: SPI,
}

impl<SPI> Emw3080<SPI>
where
    SPI: SpiDevice,
{
    /// Create a new Emw3080 driver instance.
    pub fn new(spi: SPI) -> Self {
        let mut res = Self { spi };
        res.init();
        res
    }

    fn init(&mut self) {
        // TODO...
    }
}
