//! This example prints the current temperature reading to ITM.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{pac, prelude::*},
    l3gd20, lsm303dlhc,
};

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let mut cortex_peripherals = board::peripheral::Peripherals::take()
        .expect("Cortex M peripherals are already taken at the start of the program");

    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze();
    let mut delayer = cortex_peripherals.SYST.delay(&clocks);

    let gpiob = board_peripherals.GPIOB.split();
    let mut accel =
        lsm303dlhc::Lsm303dlhc::new(board_peripherals.I2C1, gpiob.pb6, gpiob.pb9, &clocks, true)
            .expect("Failed to initialize LSM303DLHC (accelerometer + magnetometer + thermometer)");

    let gpioa = board_peripherals.GPIOA.split();
    let gpioe = board_peripherals.GPIOE.split();
    let mut gyro = l3gd20::L3gd20::new(
        board_peripherals.SPI1,
        gpioa.pa5,
        gpioa.pa6,
        gpioa.pa7,
        gpioe.pe3,
        &clocks,
    )
    .expect("Failed to initialize L3GD20 (gyroscope + thermometer)");

    let itm = &mut cortex_peripherals.ITM.stim[0];

    loop {
        let accel_temperature = {
            let t = accel
                .temp()
                .expect("Failed to read temperature from LSM303DLHC");
            // The datasheet for the thermometer doesn't specify if it's calibrated, and where its zero
            // is. Thus you can only use it to measure *change* in temperature (unless you calibrate
            // the sensor yourself).
            t as f32 / 8.0
        };
        let gyro_temperature = gyro.temp().expect("Failed to read temperature from L3GD20");
        board::iprintln!(
            itm,
            "Temperature: LSM303DLHC {:.3}C, L3GD20 {}C (both uncalibrated)",
            accel_temperature,
            gyro_temperature
        );
        delayer.delay_ms(1_000_u32);
    }
}
