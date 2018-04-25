#![feature(used)]
#![no_std]

extern crate panic_abort;

extern crate stm32f767_hal as hal;
use hal::prelude::*;
use hal::stm32f767;

#[macro_use(block)]
extern crate nb;

use hal::serial::Serial;

fn main() {
    if let Some(p) = stm32f767::Peripherals::take() {
        let gpiod = p.GPIOD.split();
        let gpiob = p.GPIOB.split();
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(108.mhz()).freeze();

        let mut led = gpiob.pb7.into_push_pull_output();

        let tx = gpiod.pd8.into_alternate_af7();
        let rx = gpiod.pd9.into_alternate_af7();

        let serial = Serial::usart3(p.USART3, (tx, rx), 115_200.bps(), clocks);

        let (mut tx, mut rx) = serial.split();

        loop {
            led.set_high();
            let received = block!(rx.read()).unwrap_or('E' as u8);
            block!(tx.write(received)).ok();

            // Turn PB7 off for a bit
            for _ in 0..1_000_000 {
                led.set_low();
            }
        }
    }
}
