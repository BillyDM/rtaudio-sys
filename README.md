# rtaudio-sys

Rust bindings for [RtAudio](https://github.com/thestk/rtaudio)

# Prerequisites

`CMake` and `Clang` are required on all platforms.

## Linux

```
apt install cmake llvm-dev libclang-dev clang pkg-config libasound2-dev libpulse-dev
```

If the `jack_linux` feature is enabled, then also install the jack development headers:
```
apt install libjack-dev
```

## MacOS

*TODO*

## Windows

### Installing Clang

#### Option 1
Using winget:

```
winget install LLVM.LLVM
```

#### Option 2

Alternatively, you can download and install the official pre-built binary from the [LLVM download page](https://releases.llvm.org/download.html).

You will also need to set `LIBCLANG_PATH` as an environment variable pointing to the bin directory of your LLVM install. For example, if you installed LLVM to `D:\programs\LLVM`, then you'd set the value to be `D:\programs\LLVM\bin`.

#### Option 3

Alternatively, for Mingw64, you can install clang via

```
pacman -S mingw64/mingw-w64-x86_64-clang
```


*TODO*

---

These bindings are auto-generated using [bindgen](https://crates.io/crates/bindgen) from the [C header](https://github.com/thestk/rtaudio/blob/master/rtaudio_c.h). No bindings to the C++ interface are provided.