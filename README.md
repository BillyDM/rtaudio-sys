# rtaudio-sys

Raw Rust bindings for [RtAudio](https://github.com/thestk/rtaudio).

This is used by the [rtaudio](https://crates.io/crates/rtaudio) crate, which provides a safe wrapper with a more Rust-y API.

Bindings are auto-generated using [bindgen](https://crates.io/crates/bindgen) from the official [C header](https://github.com/thestk/rtaudio/blob/master/rtaudio_c.h). No bindings to the C++ interface are provided.

This will build RtAudio from source. Don't forget to initialize git submodules (`git submodule update --init`) or clone with `--recursive`.