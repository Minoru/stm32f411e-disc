//! This example reads UART and sends back whatever it received; an echo server.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::hal::{
    block, pac,
    prelude::*,
    rcc,
    serial::{config::Config, Serial},
};

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(rcc::SYSCLK_MIN.Hz()).freeze();

    // USART1 is connected to (PA9, PA10) and (PB6, PB7). We can't use the first pair because PA9
    // is also used for the VBUS sensing.
    let gpiob = board_peripherals.GPIOB.split();
    let tx = gpiob.pb6;
    let rx = gpiob.pb7;

    let serial = Serial::new(
        board_peripherals.USART1,
        (tx, rx),
        Config::default(),
        &clocks,
    )
    .expect("Couldn't configure USART1");
    let (mut tx, mut rx) = serial.split();

    loop {
        let byte: u8 = block!(rx.read()).expect("Failed to read a byte");
        block!(tx.write(byte)).ok();
    }
}
