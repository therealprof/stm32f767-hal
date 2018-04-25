stm32f767-hal
=============

_stm32f767-hal_ contains a hardware abstraction on top of the peripheral access
API for the STMicro STM32F767 series microcontroller.

This crate relies on my [stm32f7][] crate to provide appropriate register
definitions and implements a partial set of the [embedded-hal][] traits.

There's a ready-made eval board I can recommend with the [nucleo-f767zi][]
on the market which even features built-in Ethernet.

Some of the implementation was shamelessly adapted from the [stm32f103xx-hal][]
crate by Jorge Aparicio.

[stm32f7]: https://crates.io/crates/stm32f7
[stm32f103xx-hal]: https://github.com/japaric/stm32f103xx-hal
[embedded-hal]: https://github.com/japaric/embedded-hal.git
[nucleo-f767zi]: https://os.mbed.com/platforms/ST-Nucleo-F767ZI/

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
