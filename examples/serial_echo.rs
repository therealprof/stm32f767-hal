#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f767_hal::{pac, prelude::*, serial::Serial};

use nb::block;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(108.mhz()).freeze();

    let gpiob = p.GPIOB.split();
    let gpiod = p.GPIOD.split();

    let mut led = gpiob.pb7.into_push_pull_output();

    let tx_pin = gpiod.pd5.into_alternate_af7();
    let rx_pin = gpiod.pd6.into_alternate_af7();

    let serial = Serial::usart2(p.USART2, (tx_pin, rx_pin), 115_200.bps(), clocks);

    let (mut tx, mut rx) = serial.split();

    loop {
        led.set_high();
        let received = block!(rx.read()).unwrap_or(b'E');
        block!(tx.write(received)).ok();

        // Turn PB7 off for a bit
        for _ in 0..2_000_000 {
            led.set_low();
        }
    }
}
