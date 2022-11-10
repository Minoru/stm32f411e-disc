//! This example lights up the blue LED as long as the blue push-button is pressed. It uses a busy
//! loop to check if the button is pressed.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::hal::{pac, prelude::*, rcc};
use board::{
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

    let gpioa = board_peripherals.GPIOA.split();
    let button = gpioa.pa0.into_input();

    let gpiod = board_peripherals.GPIOD.split();
    let blue_led = &mut Leds::new(gpiod)[LedColor::Blue];

    let delay: u32 = 10; // milliseconds

    loop {
        if button.is_high() {
            blue_led.on();
        } else {
            blue_led.off();
        }
        delayer.delay_ms(delay);
    }
}