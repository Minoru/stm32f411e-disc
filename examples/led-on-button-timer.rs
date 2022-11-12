//! This example lights up the blue LED as long as the blue push-button is pressed. It uses a timer
//! to check if the button is pressed. The main loop is thus free to do other stuff; in this
//! example, it simply sleeps waiting for the timer's interrupt.
#![no_std]
#![no_main]

use panic_halt as _;
use stm32f411e_disc as board;

use board::{
    hal::{
        pac::{self, interrupt, Interrupt, TIM2},
        prelude::*,
        rcc,
        timer::{counter::Counter, Event},
    },
    led::{LedColor, Leds},
    user_button::UserButton,
};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

#[board::entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take()
        .expect("board peripherals are already taken at the start of the program");

    let rcc = board_peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(rcc::SYSCLK_MIN.Hz()).freeze();

    let gpioa = board_peripherals.GPIOA.split();
    let button = UserButton::new(gpioa.pa0);

    let gpiod = board_peripherals.GPIOD.split();
    let leds = Leds::new(gpiod);

    // Set up a timer that will fire an interrupt every 10 ms. The timer has a frequency, which we
    // set to 1 kHz to get 1ms resolution.
    let mut timer = board_peripherals.TIM2.counter::<COUNTER_FREQ_HZ>(&clocks);
    timer.start(10.millis()).expect("Failed to start TIM2");
    timer.listen(Event::Update);

    // The interrupt handler needs access to the button, the LED, and the timer. We put those into
    // a global variable.
    cortex_m::interrupt::free(|cs| {
        let context = Tim2Context {
            leds,
            button,
            timer,
        };
        *TIM2_CONTEXT.borrow(cs).borrow_mut() = Some(context)
    });

    // Unmask the interrupt so the handler can be called when the timer expires
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    loop {
        // Wait for the interrupt
        cortex_m::asm::wfi();
    }
}

const COUNTER_FREQ_HZ: u32 = 1_000;

struct Tim2Context {
    leds: Leds,
    button: UserButton,
    timer: Counter<TIM2, COUNTER_FREQ_HZ>,
}

// The context provided to the interrupt handler.
//
// Let's break it down, from the inside out:
// - Time2Context is the context itself
// - Option reflects the fact that the context is created later in the program's runtime. Initially
//   there is no context, thus None
// - RefCell allows us to mutate the value of the global safely, because normally global statics
//   can only be mutated in `unsafe` sections
// - Mutex is a convenient wrapper that implements the `Sync` trait. It does **not** actually
//   provide synchronization between threads (which is fine because we only have one CPU core). The
//   entire purpose of `Mutex` here is to please the type checker.
static TIM2_CONTEXT: Mutex<RefCell<Option<Tim2Context>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(context) = &mut *TIM2_CONTEXT.borrow(cs).borrow_mut() {
            let blue_led = &mut context.leds[LedColor::Blue];
            if context.button.is_pressed() {
                blue_led.on();
            } else {
                blue_led.off();
            }
            context.timer.clear_interrupt(Event::Update);
        };
    })
}
