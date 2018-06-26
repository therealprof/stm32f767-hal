#![feature(used)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

extern crate panic_abort;

extern crate stm32f767_hal as hal;
use hal::prelude::*;
use hal::stm32f767;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

entry!(main);

fn main() -> ! {
    if let Some(p) = stm32f767::Peripherals::take() {
        let gpiob = p.GPIOB.split();

        // Configure PB7 as output
        let mut led = gpiob.pb7.into_push_pull_output();

        loop {
            // Turn PB7 on a million times in a row
            for _ in 0..1_000_000 {
                led.set_high();
            }
            // Then turn PB7 off a million times in a row
            for _ in 0..1_000_000 {
                led.set_low();
            }
        }
    }

    loop {}
}
