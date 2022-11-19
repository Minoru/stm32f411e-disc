//! L3GD20, gyroscope + uncalibrated thermometer.
//!
//! This is a thin wrapper over the l3gd20 crate. Please see its docs for available methods.

use crate::hal::{
    gpio::{self, PA5, PA6, PA7, PE3},
    pac::SPI1,
    prelude::*,
    rcc::Clocks,
    spi::{Error, Spi},
};

type Inner = l3gd20::L3gd20<Spi<SPI1, (PA5, PA6, PA7)>, PE3<gpio::Output>>;

/// Gyroscope and thermometer.
pub struct L3gd20 {
    inner: Inner,
}

impl L3gd20 {
    /// Initialize the sensor.
    pub fn new(
        spi1: SPI1,
        sck: PA5,
        miso: PA6,
        mosi: PA7,
        ncs: PE3,
        clocks: &Clocks,
    ) -> core::result::Result<Self, Error> {
        let spi = Spi::new(spi1, (sck, miso, mosi), l3gd20::MODE, 10_u32.MHz(), clocks);
        let ncs = ncs.into_push_pull_output();
        l3gd20::L3gd20::new(spi, ncs).map(|inner| Self { inner })
    }
}

impl core::ops::Deref for L3gd20 {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for L3gd20 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
