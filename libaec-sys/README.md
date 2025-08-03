# libaec-sys

[![docs](https://docs.rs/libaec-sys/badge.svg)](https://docs.rs/libaec-sys)
[![Crates.io](https://img.shields.io/crates/v/libaec-sys)](https://crates.io/crates/libaec-sys)

This crate provides low-level bindings to [Adaptive Entropy Coding library (libaec)](https://gitlab.dkrz.de/k202009/libaec).

The description of libaec is as follows:

> Libaec provides fast lossless compression of 1 up to 32 bit wide
> signed or unsigned integers (samples). The library achieves best
> results for low entropy data as often encountered in space imaging
> instrument data or numerical model output from weather or climate
> simulations. While floating point representations are not directly
> supported, they can also be efficiently coded by grouping exponents
> and mantissa.
>
> ## Scope
>
> Libaec implements extended
> [Golomb-Rice](http://en.wikipedia.org/wiki/Golomb_coding) coding as
> defined in the CCSDS recommended standard [121.0-B-3][1]. The library
> covers the adaptive entropy coder and the preprocessor discussed in
> sections 1 to 5.2.6 of the [standard][1].
>
> [1]: https://public.ccsds.org/Pubs/121x0b3.pdf

## License

This project is licensed under either of

 * Apache License, Version 2.0 (See [LICENSE-APACHE](LICENSE-APACHE)
   or http://www.apache.org/licenses/LICENSE-2.0), and
 * MIT license (See [LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

`SPDX-License-Identifier: Apache-2.0 OR MIT`
