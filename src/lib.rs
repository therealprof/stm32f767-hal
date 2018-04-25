#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]
#![feature(never_type)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
pub extern crate embedded_hal as hal;
#[macro_use]
pub extern crate nb;
pub use nb::block;
pub extern crate stm32f7;

pub use stm32f7::stm32f7x7 as stm32f767;

pub mod delay;
pub mod gpio;
pub mod i2c;
pub mod prelude;
pub mod rcc;
pub mod serial;
pub mod time;
