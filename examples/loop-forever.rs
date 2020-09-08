#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

#[board::entry]
fn main() -> ! {
    loop {}
}
