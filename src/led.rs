//! On-board user LEDs.

use crate::hal::gpio::gpiod::{PD12, PD13, PD14, PD15};
use crate::hal::gpio::{Output, PinMode, PushPull};

use switch_hal::{ActiveHigh, IntoSwitch, Switch};

pub use switch_hal::OutputSwitch;

// TODO: provide a convenient interface to get all LEDs at once:
//
//  fn helper(&mut gpiod) -> (green, orange, red, blue)
//
// TODO: provide a trait `Led` so users (like led-roulette) can make an array of LEDs and iterate
// over them.
//
// TODO: perhaps replace into_<colour>_led() with a single into_led()? The benefits of the multiple
// names are marginal, what are the chances that the user doesn't know which colour the LED they're
// trying to obtain? What's the cost of that mistake compared to the effort of learning four
// functions instead of one?

/// Create a switch for the green LED (`LD4`).
pub fn into_green_led<T>(pin: PD12<T>) -> Switch<PD12<Output<PushPull>>, ActiveHigh>
where
    T: PinMode,
{
    pin.into_push_pull_output().into_active_high_switch()
}

/// Create a switch for the orange LED (`LD3`).
pub fn into_orange_led<T>(pin: PD13<T>) -> Switch<PD13<Output<PushPull>>, ActiveHigh>
where
    T: PinMode,
{
    pin.into_push_pull_output().into_active_high_switch()
}

/// Create a switch for the red LED (`LD5`).
pub fn into_red_led<T>(pin: PD14<T>) -> Switch<PD14<Output<PushPull>>, ActiveHigh>
where
    T: PinMode,
{
    pin.into_push_pull_output().into_active_high_switch()
}

/// Create a switch for the blue LED (`LD6`).
pub fn into_blue_led<T>(pin: PD15<T>) -> Switch<PD15<Output<PushPull>>, ActiveHigh>
where
    T: PinMode,
{
    pin.into_push_pull_output().into_active_high_switch()
}
