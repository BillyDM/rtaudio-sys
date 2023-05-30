# rtaudio-sys

Rust bindings for [RtAudio](https://github.com/thestk/rtaudio)

# Prerequisites

`CMake` and `Clang` are required on all platforms.

## Linux

```
apt install libasound2-dev libpulse-dev
```

If the `jack_linux` feature is enabled, then also install the jack development headers:
```
apt install libjack-dev
```

## MacOS

*TODO*

## Windows

*TODO*

---

These bindings are auto-generated using [bindgen](https://crates.io/crates/bindgen) from the [C header](https://github.com/thestk/rtaudio/blob/master/rtaudio_c.h). No bindings to the C++ interface are provided.