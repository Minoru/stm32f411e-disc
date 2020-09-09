#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::hal::{delay, prelude::*, stm32};
use board::peripheral;

#[board::entry]
fn main() -> ! {
    let board_peripherals = stm32::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let cortex_peripherals = peripheral::Peripherals::take()
        .expect("Cortex M peripherals are already taken at the start of the program");

    let syst = cortex_peripherals.SYST;
    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delayer = delay::Delay::new(syst, clocks);

    // User-programmable LEDs are connected to pins PD12 through PD15.
    let gpiod = board_peripherals.GPIOD.split();
    let mut green = gpiod.pd12.into_push_pull_output(); // LD4
    let mut orange = gpiod.pd13.into_push_pull_output(); // LD3
    let mut red = gpiod.pd14.into_push_pull_output(); // LD4
    let mut blue = gpiod.pd15.into_push_pull_output(); // LD5

    let delay: u32 = 100; // milliseconds

    loop {
        blue.set_low().expect("failed to set blue LED to low");
        green.set_high().expect("failed to set green LED to high");
        delayer.delay_ms(delay);

        green.set_low().expect("failed to set green LED to high");
        orange.set_high().expect("failed to set orange LED to high");
        delayer.delay_ms(delay);

        orange.set_low().expect("failed to set orange LED to high");
        red.set_high().expect("failed to set red LED to high");
        delayer.delay_ms(delay);

        red.set_low().expect("failed to set red LED to high");
        blue.set_high().expect("failed to set blue LED to high");
        delayer.delay_ms(delay);
    }
}
