#![feature(used)]
#![no_std]

extern crate panic_abort;

extern crate stm32f767_hal as hal;
use hal::gpio::*;
use hal::prelude::*;
use hal::stm32f767;

#[macro_use(exception)]
extern crate stm32f7;

extern crate cortex_m;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::syst::SystClkSource::Core;
use cortex_m::peripheral::Peripherals;

use core::cell::RefCell;
use core::ops::DerefMut;

static GPIO: Mutex<RefCell<Option<gpiob::PB7<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

fn main() {
    if let (Some(p), Some(cp)) = (stm32f767::Peripherals::take(), Peripherals::take()) {
        let gpiob = p.GPIOB.split();
        let mut rcc = p.RCC.constrain();
        let _ = rcc.cfgr.sysclk(100.mhz()).freeze();
        let mut syst = cp.SYST;

        /* (Re-)configure Pb7 as output */
        let mut led = gpiob.pb7.into_push_pull_output();

        cortex_m::interrupt::free(move |cs| {
            *GPIO.borrow(cs).borrow_mut() = Some(led);
        });

        /* Initialise SysTick counter with a defined value */
        unsafe { syst.cvr.write(1) };

        /* Set source for SysTick counter, here full operating frequency (== 8MHz) */
        syst.set_clock_source(Core);

        /* Set reload value, i.e. timer delay 100 MHz/4 Mcounts == 25Hz or 40ms */
        syst.set_reload(4_000_000 - 1);

        /* Start counter */
        syst.enable_counter();

        /* Start interrupt generation */
        syst.enable_interrupt();
    }
}

/* Define an exception, i.e. function to call when exception occurs. Here if our SysTick timer
 * trips the flash function will be called and the specified stated passed in via argument */
exception!(SYS_TICK, flash, locals: {
    state: u8 = 1;
});

fn flash(l: &mut SYS_TICK::Locals) {
    /* Enter critical section */
    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut led) = GPIO.borrow(cs).borrow_mut().deref_mut() {
            /* Check state variable, keep LED off most of the time and turn it on every 10th tick */
            if l.state < 10 {
                // If set turn off the LED
                led.set_low();

                // And now increment state variable
                l.state += 1;
            } else {
                // If not set, turn on the LED
                led.set_high();

                // And set new state variable back to 0
                l.state = 0;
            }
        }
    });
}
