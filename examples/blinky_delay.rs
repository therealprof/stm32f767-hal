#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f767_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the RCC peripheral and convert it to the
    // corresponding HAL struct.
    let rcc = p.RCC.constrain();

    // Configure the clock and freeze it.
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    // Acquire the GBIOB peripheral. This also enables the clock for GPIOB in
    // the RCC register.
    let gpiob = p.GPIOB.split();

    // Configure PB0 as output.
    let mut led = gpiob.pb0.into_push_pull_output();

    // Get the delay provider.
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(500_u16);

        led.set_low();
        delay.delay_ms(500_u16);
    }
}
