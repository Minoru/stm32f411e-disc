//! This example reports gyroscope measurements via ITM. Due to noticeable drift, we can't build
//! a more interesting example.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{pac, prelude::*},
    l3gd20,
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
    let scale = gyro.scale().expect("Failed to get L3GD20 scale");

    let itm = &mut cortex_peripherals.ITM.stim[0];
    loop {
        if let Ok(measurement) = gyro.gyro() {
            board::iprintln!(
                itm,
                "L3GD20 measurements:\tx {:10.6} deg/s\ty {:10.6} deg/s\tz {:10.6} deg/s",
                scale.degrees(measurement.x),
                scale.degrees(measurement.y),
                scale.degrees(measurement.z),
            )
        }

        delayer.delay_ms(1_000_u32);
    }
}
