#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_halt;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use cortex_m::{asm, peripheral::syst};
use cortex_m_rt::{entry, exception};
use stm32f767_hal::{gpio::*, pac, prelude::*};

// The LED is initialised in the main, but we need to access it from the SysTick
// exception handler. Using a Mutex let us do this in a safe way. The RefCell is
// mandatory to make the LED mutable in a non-mutable Mutex container.
static LED: Mutex<RefCell<Option<gpiob::PB7<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the RCC peripheral and convert it to the
    // corresponding HAL struct.
    let rcc = p.RCC.constrain();

    // Configure the clock and freeze it.
    rcc.cfgr.sysclk(100.mhz()).freeze();

    // Acquire the GBIOB peripheral. This also enables the clock for GPIOB in
    // the RCC register.
    let gpiob = p.GPIOB.split();

    // Configure PB7 as output.
    let led = gpiob.pb7.into_push_pull_output();

    // Enter critical section to access the Mutex.
    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs).replace(Some(led));
    });

    let mut systick = cp.SYST;

    // Set the clock source for the SysTick counter.
    systick.set_clock_source(syst::SystClkSource::Core);

    // Set reload value, i.e. timer delay 100 MHz/4 Mcounts == 25Hz or 40ms.
    systick.set_reload(4_000_000 - 1);

    // Start the counter.
    systick.enable_counter();

    // Enable systick interrupt generation.
    systick.enable_interrupt();

    loop {
        // Wait for an interrupt.
        asm::wfi();
    }
}

// Define an exception handler for the SysTick.
#[exception]
fn SysTick() {
    static mut STATE: u8 = 0;

    // Enter critical section to access the Mutex.
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut led) = *LED.borrow(cs).borrow_mut() {
            // Check state variable, keep LED off most of the time and turn it
            // on every 10th tick.

            if *STATE < 10 {
                // If set turn off the LED.
                led.set_low();

                // And now increment state variable.
                *STATE += 1;
            } else {
                // If not set, turn on the LED.
                led.set_high();

                // And set new state variable back to 0.
                *STATE = 0;
            }
        }
    });
}
