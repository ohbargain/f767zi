//! Blinks an LED
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f767zi;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
// extern crate cortex_m_semihosting as semihosting;

use cortex_m::peripheral::SystClkSource;
use f767zi::led::{self, LEDS};
use rtfm::{app, Threshold};
// use core::fmt::Write;
// use semihosting::hio;


// CONFIGURATION
const FREQUENCY: u32 = 1; // Hz

// TASKS & RESOURCES
app! {
    device: f767zi::stm32f7x7,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOB, p.RCC);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000 / FREQUENCY);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// fn sayHello(_t: &mut Threshold, r: SYS_TICK::Resources) {
// // fn sayHello() {
//     writeln!(hio::hstdout().unwrap(), "Hello, world!").unwrap();
// }
// TASKS
// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        LEDS[2].on();
        LEDS[0].on();
    } else {
        LEDS[2].off();
        LEDS[0].off();
    }
}
