//! User button (the blue one).
//!
//! ```no_run
//! use stm32f411e_disc as board;
//!
//! #[board::entry]
//! fn main() -> ! {
//!     let board_peripherals = board::hal::pac::Peripherals::take()
//!         .expect("Failed to take the board peripherals");
//!     let gpioa = board_peripherals.GPIOA.split();
//!     let button = UserButton::new(gpioa.pa0);
//!     if button.is_pressed() {
//!         // do stuff
//!     }
//! }
//! ```

use crate::hal::gpio::gpioa::PA0;

/// User button (the blue one).
pub struct UserButton {
    /// The pin to which the button is connected to.
    ///
    /// This is public in order to give access to various other methods on the pin, e.g. EXTI
    /// interrupts.
    pub pin: PA0,
}

impl UserButton {
    /// Create the abstraction from the pin connected to the button.
    pub fn new(pa0: PA0) -> Self {
        UserButton {
            pin: pa0.into_input(),
        }
    }

    /// Is the button pressed?
    pub fn is_pressed(&self) -> bool {
        self.pin.is_high()
    }
}
