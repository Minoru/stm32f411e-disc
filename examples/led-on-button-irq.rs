//! This example lights up the blue LED as long as the blue push-button is pressed. It uses an
//! interrupt that occurs whenever the button is pressed. The main loop is thus free to do other
//! stuff; in this example, it simply sleeps waiting for the button's interrupt.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{
        gpio::{gpioa::PA0, Edge},
        pac::{self, interrupt, Interrupt},
        prelude::*,
    },
    led::{LedColor, Leds},
};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    // Get the pin that corresponds to the button
    let gpioa = board_peripherals.GPIOA.split();
    let mut button = gpioa.pa0.into_input();

    // Configure the button to generate EXTI interrupt
    let mut syscfg = board_peripherals.SYSCFG.constrain();
    let mut exti = board_peripherals.EXTI;
    button.make_interrupt_source(&mut syscfg);
    // Button connects voltage to the pin. The interrupt is generated whenever the voltage changes.
    // Here, we request an interrupt to be generated *both* when the voltage is applied (a rising
    // edge) and when the voltage stops being applied (a falling edge)
    button.trigger_on_edge(&mut exti, Edge::RisingFalling);
    button.enable_interrupt(&mut exti);

    let gpiod = board_peripherals.GPIOD.split();
    let leds = Leds::new(gpiod);

    // The interrupt handler needs access to the button and the LED. We put those into a global
    // variable.
    cortex_m::interrupt::free(|cs| {
        let context = IrqContext {
            leds,
            button,
            was_pressed: false,
        };
        *IRQ_CONTEXT.borrow(cs).borrow_mut() = Some(context)
    });

    // Unmask the interrupt so the handler can be called when the interrupt fires
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::EXTI0);
    }

    loop {
        // Wait for the interrupt
        cortex_m::asm::wfi();
    }
}

struct IrqContext {
    leds: Leds,

    button: PA0,

    /// The state of the button the last time the interrupt fired.
    ///
    /// The interrupt handler compares this to the current state of the button in order to decide
    /// if it should turn the LED on or off.
    was_pressed: bool,
}

// The context provided to the interrupt handler.
//
// Let's break it down, from the inside out:
// - IrqContext is the context itself
// - Option reflects the fact that the context is created later in the program's runtime. Initially
//   there is no context, thus None
// - RefCell allows us to mutate the value of the global safely, because normally global statics
//   can only be mutated in `unsafe` sections
// - Mutex is a convenient wrapper that implements the `Sync` trait. It does **not** actually
//   provide synchronization between threads (which is fine because we only have one CPU core). The
//   entire purpose of `Mutex` here is to please the type checker.
static IRQ_CONTEXT: Mutex<RefCell<Option<IrqContext>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(context) = &mut *IRQ_CONTEXT.borrow(cs).borrow_mut() {
            let blue_led = &mut context.leds[LedColor::Blue];
            let is_pressed = context.button.is_high();
            if !context.was_pressed && is_pressed {
                blue_led.on();
            } else if context.was_pressed && !is_pressed {
                blue_led.off();
            }
            context.was_pressed = is_pressed;
            context.button.clear_interrupt_pending_bit();
        };
    })
}
