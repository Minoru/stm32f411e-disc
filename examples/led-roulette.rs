#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::hal::{delay, prelude::*, stm32};
use board::{
    led::{self, LedColor},
    peripheral,
};

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

    let order = [
        LedColor::Orange,
        LedColor::Red,
        LedColor::Blue,
        LedColor::Green,
    ];

    let mut leds = led::Leds::new(board_peripherals.GPIOD.split());

    for (prev, cur) in order.iter().cycle().zip(order.iter().cycle().skip(1)) {
        leds[*prev].off();
        leds[*cur].on();
        delayer.delay_ms(100u32);
    }

    unreachable!("The infinite loop over LEDS terminated");
}
