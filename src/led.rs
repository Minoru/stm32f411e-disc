//! On-board user LEDs.

use crate::hal::gpio::gpiod::{self, PDn, PD12, PD13, PD14, PD15};
use crate::hal::gpio::{Output, PushPull};

/// Top LED (orange).
pub type LD3 = PD13<Output<PushPull>>;

/// Left LED (green).
pub type LD4 = PD12<Output<PushPull>>;

/// Right LED (red).
pub type LD5 = PD14<Output<PushPull>>;

/// Bottom LED (blue).
pub type LD6 = PD15<Output<PushPull>>;

/// User LED colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedColor {
    /// Top/orange LED (`LD3`).
    Orange,
    /// Left/green LED (`LD4`).
    Green,
    /// Right/red LED (`LD5`).
    Red,
    /// Bottom/blue LED (`LD6`).
    Blue,
}

/// Array of on-board user LEDs.
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let ld3 = gpiod.pd13.into_push_pull_output();
        let ld4 = gpiod.pd12.into_push_pull_output();
        let ld5 = gpiod.pd14.into_push_pull_output();
        let ld6 = gpiod.pd15.into_push_pull_output();

        Leds {
            leds: [ld3.into(), ld4.into(), ld5.into(), ld6.into()],
        }
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Led> {
        self.leds.iter_mut()
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for Leds {
    type Output = Led;

    fn index(&self, color: LedColor) -> &Led {
        &self.leds[color as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for Leds {
    fn index_mut(&mut self, color: LedColor) -> &mut Led {
        &mut self.leds[color as usize]
    }
}

/// An on-board user LED.
pub struct Led {
    pin: PDn<Output<PushPull>>,
}

impl Led {
    /// Turn the LED on.
    pub fn on(&mut self) {
        self.pin.set_high();
    }

    /// Turn the LED off.
    pub fn off(&mut self) {
        self.pin.set_low();
    }

    /// Toggle the LED.
    pub fn toggle(&mut self) {
        if self.pin.is_set_low() {
            self.on()
        } else {
            self.off()
        }
    }
}

impl From<LD3> for Led {
    fn from(ld: LD3) -> Self {
        Led {
            pin: ld.erase_number(),
        }
    }
}

impl From<LD4> for Led {
    fn from(ld: LD4) -> Self {
        Led {
            pin: ld.erase_number(),
        }
    }
}

impl From<LD5> for Led {
    fn from(ld: LD5) -> Self {
        Led {
            pin: ld.erase_number(),
        }
    }
}

impl From<LD6> for Led {
    fn from(ld: LD6) -> Self {
        Led {
            pin: ld.erase_number(),
        }
    }
}
