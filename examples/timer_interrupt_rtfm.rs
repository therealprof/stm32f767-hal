#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_halt;

use rtfm::app;
use stm32f767_hal::{
    gpio::*,
    pac,
    prelude::*,
    timer::{Event, Timer},
};

#[app(device = stm32f767_hal::pac)]
const APP: () = {
    static mut LED: gpiob::PB0<Output<PushPull>> = ();
    static mut TIMER: Timer<pac::TIM2> = ();

    #[init]
    fn init() -> init::LateResources {
        // Take & convert the RCC peripheral to its HAL struct.
        let rcc = device.RCC.constrain();

        // Configure the clock.
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        // Acquire the GBIOB peripheral. This also enables the clock for GPIOB
        // in the RCC register.
        let gpiob = device.GPIOB.split();

        // Configure PB0 as output.
        let led = gpiob.pb0.into_push_pull_output();

        // Configure the timer.
        let mut timer = Timer::tim2(device.TIM2, 1.hz(), clocks);
        timer.listen(Event::TimeOut);

        // Return the initialised resources.
        init::LateResources {
            LED: led,
            TIMER: timer,
        }
    }

    #[interrupt(resources = [LED, TIMER])]
    fn TIM2() {
        static mut STATE: bool = false;

        // Clear the interrupt flag.
        resources.TIMER.clear_update_interrupt_flag();

        if *STATE {
            resources.LED.set_low();
            *STATE = false;
        } else {
            resources.LED.set_high();
            *STATE = true;
        }
    }
};
