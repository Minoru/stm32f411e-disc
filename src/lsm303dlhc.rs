//! LSM303DLHC, accelerometer + magnetometer.
//!
//! This is a thin wrapper over the lsm303dlhc crate. Please see its docs for available methods.

use crate::hal::{
    gpio::{PB6, PB9},
    i2c::{Error, I2c, Mode},
    pac::I2C1,
    prelude::*,
    rcc::Clocks,
};

type Inner = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6, PB9)>>;

/// Accelerometer and magnetometer.
pub struct Lsm303dlhc {
    pub inner: Inner,
}

impl Lsm303dlhc {
    /// Initialize the sensor.
    ///
    /// If `high_frequency` is set, the I²C bus will run at 400 kHz; otherwise it'll run at 100 kHz.
    pub fn new(
        i2c1: I2C1,
        scl: PB6,
        sda: PB9,
        clocks: &Clocks,
        high_frequency: bool,
    ) -> core::result::Result<Self, Error> {
        let i2c1 = I2c::new(
            i2c1,
            (scl, sda),
            Mode::standard(if high_frequency {
                400_u32.kHz()
            } else {
                100_u32.kHz()
            }),
            clocks,
        );
        lsm303dlhc::Lsm303dlhc::new(i2c1).map(|inner| Self { inner })
    }
}

impl core::ops::Deref for Lsm303dlhc {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Lsm303dlhc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
