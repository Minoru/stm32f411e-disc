//! This example reports accelerometer measurements via ITM.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{pac, prelude::*},
    lsm303dlhc,
};

use libm::sqrtf;

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
    let mut sensor =
        lsm303dlhc::Lsm303dlhc::new(board_peripherals.I2C1, gpiob.pb6, gpiob.pb9, &clocks, true)
            .expect("Failed to initialize LSM303DLHC (accelerometer + magnetometer)");
    sensor
        .set_accel_sensitivity(::lsm303dlhc::Sensitivity::G1)
        .expect("Failed to reconfigure LSM303DLHC sensitivity");

    let itm = &mut cortex_peripherals.ITM.stim[0];
    loop {
        if let Ok(measurement) = sensor.accel() {
            let x = measurement.x as f32 / (2_i32.pow(14) as f32);
            let y = measurement.y as f32 / (2_i32.pow(14) as f32);
            let z = measurement.z as f32 / (2_i32.pow(14) as f32);
            let a = sqrtf(x * x + y * y + z * z);
            board::iprintln!(
                itm,
                "LSM303DLHC measurements:\tx {:6.3} g\ty {:6.3} g\tz {:6.3} g\ta {:6.3} g",
                x,
                y,
                z,
                a
            )
        }

        delayer.delay_ms(1_000_u32);
    }
}
