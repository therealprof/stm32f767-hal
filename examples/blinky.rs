#![feature(used)]
#![no_std]

extern crate panic_abort;

extern crate stm32f767_hal as hal;
use hal::prelude::*;
use hal::stm32f767;

fn main() {
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
}
