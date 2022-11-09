#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::hal::{pac, prelude::*};
use board::{
    led::{self, OutputSwitch},
    peripheral,
};

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let cortex_peripherals = peripheral::Peripherals::take()
        .expect("Cortex M peripherals are already taken at the start of the program");

    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delayer = cortex_peripherals.SYST.delay(&clocks);

    let gpiod = board_peripherals.GPIOD.split();

    let mut green_led = led::into_green_led(gpiod.pd12);
    let mut orange_led = led::into_orange_led(gpiod.pd13);
    let mut red_led = led::into_red_led(gpiod.pd14);
    let mut blue_led = led::into_blue_led(gpiod.pd15);

    let delay: u32 = 100; // milliseconds

    loop {
        green_led.off().ok(); // TODO: replace by into_ok() once it's stabilized https://github.com/rust-lang/rust/issues/61695
        orange_led.on().ok();
        delayer.delay_ms(delay);

        orange_led.off().ok();
        red_led.on().ok();
        delayer.delay_ms(delay);

        red_led.off().ok();
        blue_led.on().ok();
        delayer.delay_ms(delay);

        blue_led.off().ok();
        green_led.on().ok();
        delayer.delay_ms(delay);
    }
}
