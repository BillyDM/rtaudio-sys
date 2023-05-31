# rtaudio-sys

Rust bindings for [RtAudio](https://github.com/thestk/rtaudio)

# Prerequisites

`CMake` and `Clang` are required on all platforms.

## Linux

```
apt install cmake clang pkg-config libasound2-dev libpulse-dev
```

If the `jack_linux` feature is enabled, then also install the jack development headers:
```
apt install libjack-dev
```

## MacOS

### Install CMake: Option 1

Download at https://cmake.org/.

### Install CMake: Option 2

Install with [Homebrew](https://brew.sh/):

```
brew install cmake
```

## Windows

### Install Clang: Option 1
Install with winget:

```
winget install LLVM.LLVM
```

### Install Clang: Option 2

Alternatively, you can download and install the official pre-built binary from the [LLVM download page](https://releases.llvm.org/download.html).

You will also need to set `LIBCLANG_PATH` as an environment variable pointing to the bin directory of your LLVM install. For example, if you installed LLVM to `D:\programs\LLVM`, then you'd set the value to be `D:\programs\LLVM\bin`.

### Install Clang: Option 3

Alternatively, for Mingw64, you can install clang via:

```
pacman -S mingw64/mingw-w64-x86_64-clang
```

### Install CMake

Download at https://cmake.org/.

# Features

By default, Jack on Linux and ASIO on Windows is disabled. You can enable them with the `jack_linux` and `asio` features.

```
rtaudio-sys = { version = "0.1", features = ["jack_linux", "asio"] }
```

# Notes

Bindings are auto-generated using [bindgen](https://crates.io/crates/bindgen) from the official [C header](https://github.com/thestk/rtaudio/blob/master/rtaudio_c.h). No bindings to the C++ interface are provided.

This currently builds a static library from source on all platforms. Once RtAudio version 6 stabilizes I might have it link to the dynamic RtAudio library on Linux.

I haven't figured out how to get Jack on MacOS to work yet. If you know how to install and link the Jack libraries on MacOS, please let me know.

I haven't thoroughly tested every API on every platform yet. If you run into any bugs or issues with building, please create an issue.