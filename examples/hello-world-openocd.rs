//! This example prints "Hello, world!" to the OpenOCD console.
//!
//! In order to see it, you'll have to run a couple commands:
//!
//! 1. `cargo build --example hello-world-openocd --release` to build this example;
//! 2. `openocd` from the root of this crate's repository, to start OpenOCD;
//! 3. *from a different terminal*: `gdb-multiarch -q
//!    target/thumbv7em-none-eabihf/release/examples/hello-world-openocd`, which will start a GDB
//!    session in which you have to run a few more GDB commands:
//!
//!    1. `target extended-remote :3333` to connect to OpenOCD;
//!    2. `load` to flash the program onto the board;
//!    3. `monitor arm semihosting enable` to enable semihosting;
//!    4. `continue` to start running the program.
//!
//! After this you'll see "Hello, world!" in OpenOCD output.
//!
//! To exit GDB, hold down Control key and press c, then type `exit` and press y then Enter to
//! confirm. To exit OpenOCD, hold down Control key and press c.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use cortex_m_semihosting::hprintln;

#[board::entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    loop {}
}
