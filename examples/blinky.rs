#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f767_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Acquire the GBIOB peripheral. This also enables the clock for GPIOB in
    // the RCC register.
    let gpiob = p.GPIOB.split();

    // Configure PB7 as output.
    let mut led = gpiob.pb7.into_push_pull_output();

    loop {
        // Turn PB7 on a million times in a row.
        for _ in 0..1_000_000 {
            led.set_high();
        }

        // Then turn PB7 off a million times in a row.
        for _ in 0..1_000_000 {
            led.set_low();
        }
    }
}
