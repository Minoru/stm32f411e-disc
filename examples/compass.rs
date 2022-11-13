//! This example lights up the LED that points to the magnetic north. It also prints current
//! magnetic measurements to ITM (see hello-world-itm.rs for instructions on how to read ITM).
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{pac, prelude::*},
    led::{LedColor, Leds},
    lsm303dlhc,
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

    let gpiod = board_peripherals.GPIOD.split();

    let mut leds = Leds::new(gpiod);

    let gpiob = board_peripherals.GPIOB.split();
    let mut sensor =
        lsm303dlhc::Lsm303dlhc::new(board_peripherals.I2C1, gpiob.pb6, gpiob.pb9, &clocks, true)
            .expect("Failed to initialize LSM303DLHC (accelerometer + magnetometer)");
    let itm = &mut cortex_peripherals.ITM.stim[0];

    let delay: u32 = 100; // milliseconds

    loop {
        let mag = sensor
            .mag()
            .expect("Failed to read magnetometer measurements from LSM303DLHC");
        board::iprintln!(itm, "Magnetometer measurements: {:?}", mag);

        for led in leds.iter_mut() {
            led.off();
        }

        if mag.x.abs() > mag.y.abs() {
            if mag.x > 0 {
                leds[LedColor::Orange].on();
            } else {
                leds[LedColor::Blue].on();
            }
        } else {
            if mag.y > 0 {
                leds[LedColor::Green].on();
            } else {
                leds[LedColor::Red].on();
            }
        }

        delayer.delay_ms(delay);
    }
}
