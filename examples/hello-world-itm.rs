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
//!    target/thumbv7em-none-eabihf/release/examples/hello-world-itm`, which will start a GDB
//!    session in which you have to run a few more GDB commands:
//!
//!    1. `target extended-remote :3333` to connect to OpenOCD;
//!    2. `monitor tpiu config internal itm.txt uart off 16000000` to start writing ITM messages
//!       into itm.txt file. 16000000 is 16 MHz, the default core frequency of our Discovery board;
//!    3. `monitor itm port 0 on` to enable the ITM port;
//!    4. `load` to flash the program onto the board;
//!    5. `continue` to start running the program.
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

    loop {}
}
