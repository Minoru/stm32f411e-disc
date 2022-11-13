//! This example prints "Hello, world!" through the ITM.
//!
//! In order to see it, you'll have to run a couple commands:
//!
//! 1. `cargo install itm` to install `itmdump` utility;
//! 2. `cargo build --example hello-world-itm --release` to build this example;
//! 3. `touch itm.txt && itmdump --follow --file itm.txt` to create a new text file and start dumping its
//!    contents with `itmdump`;
//! 4. *in a different terminal*, `openocd` to launch OpenOCD;
//! 5. *in yet another terminal*, `gdb-multiarch -q
//!    target/thumbv7em-none-eabihf/release/examples/hello-world-itm -x openocd-itm.gdb`, which
//!    will start a GDB session with all the necessary configurations;
//! 6. in GDB, type `continue` to start the program.
//!
//! You should now see output in the terminal where `itmdump` is running.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

#[board::entry]
fn main() -> ! {
    let cortex_peripherals = board::peripheral::Peripherals::take()
        .expect("Cortex M peripherals are already taken at the start of the program");

    let mut itm = cortex_peripherals.ITM;
    board::iprintln!(&mut itm.stim[0], "Hello, world!");

    #[allow(clippy::empty_loop)]
    loop {}
}
