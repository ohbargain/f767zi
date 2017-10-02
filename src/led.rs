//! User LEDs

use stm32f7x7::{GPIOB, RCC};

/// All the user LEDs
pub static LEDS: [Led; 3] = [Led { i: 0 }, Led { i: 7 }, Led { i: 14 }];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOB.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOB.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpioe: &GPIOB, rcc: &RCC) {
    // Power up peripherals
    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

    gpioe.moder.modify(
        |_, w|
        unsafe {
              w.moder0().bits(1).moder7().bits(1).moder14().bits(1)
        }
    );

}
