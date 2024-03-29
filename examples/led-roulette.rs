//! This example lights up each user LED in turn, endlessly.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{pac, prelude::*, rcc},
    led::{LedColor, Leds},
    peripheral,
};

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let cortex_peripherals = peripheral::Peripherals::take()
        .expect("Cortex M peripherals are already taken at the start of the program");

    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(rcc::SYSCLK_MIN.Hz()).freeze();
    let mut delayer = cortex_peripherals.SYST.delay(&clocks);

    let gpiod = board_peripherals.GPIOD.split();

    let mut leds = Leds::new(gpiod);
    let colors = [
        LedColor::Orange,
        LedColor::Green,
        LedColor::Blue,
        LedColor::Red,
    ];

    let delay: u32 = 100; // milliseconds

    loop {
        for color in colors {
            leds[color].on();
            delayer.delay_ms(delay);
            leds[color].off();
        }
    }
}
