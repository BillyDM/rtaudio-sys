extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    // Build the static library with CMake.
    let mut config = cmake::Config::new("rtaudio");
    config.uses_cxx11();

    #[cfg(target_os = "linux")]
    {
        #[cfg(feature = "alsa")]
        config.define("RTAUDIO_API_ALSA", "ON");
        #[cfg(not(feature = "alsa"))]
        config.define("RTAUDIO_API_ALSA", "OFF");

        #[cfg(feature = "pulse")]
        config.define("RTAUDIO_API_PULSE", "ON");
        #[cfg(not(feature = "pulse"))]
        config.define("RTAUDIO_API_PULSE", "OFF");

        #[cfg(feature = "jack_linux")]
        config.define("RTAUDIO_API_JACK", "ON");
        #[cfg(not(feature = "jack_linux"))]
        config.define("RTAUDIO_API_JACK", "OFF");
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        #[cfg(feature = "oss")]
        config.define("RTAUDIO_API_OSS", "ON");
        #[cfg(not(feature = "oss"))]
        config.define("RTAUDIO_API_OSS", "OFF");
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(feature = "coreaudio")]
        config.define("RTAUDIO_API_CORE", "ON");
        #[cfg(not(feature = "coreaudio"))]
        config.define("RTAUDIO_API_CORE", "OFF");

        #[cfg(feature = "jack_macos")]
        config.define("RTAUDIO_API_JACK", "ON");
        #[cfg(not(feature = "jack_macos"))]
        config.define("RTAUDIO_API_JACK", "OFF");
    }

    #[cfg(target_os = "windows")]
    {
        #[cfg(feature = "ds")]
        config.define("RTAUDIO_API_DS", "ON");
        #[cfg(not(feature = "ds"))]
        config.define("RTAUDIO_API_DS", "OFF");

        #[cfg(feature = "asio")]
        config.define("RTAUDIO_API_ASIO", "ON");
        #[cfg(not(feature = "asio"))]
        config.define("RTAUDIO_API_ASIO", "OFF");

        #[cfg(feature = "wasapi")]
        config.define("RTAUDIO_API_WASAPI", "ON");
        #[cfg(not(feature = "wasapi"))]
        config.define("RTAUDIO_API_WASAPI", "OFF");
    }

    let dst = config.build();

    let mut libdir_path = dst.clone();
    libdir_path.push("lib");

    // Tell cargo to link to the compiled library.
    println!(
        "cargo:rustc-link-search=native={}",
        libdir_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=rtaudio");

    let mut headers_path = dst;
    headers_path.push("include/rtaudio/rtaudio_c.h");

    // Generate Rust bindings from the C header.
    let bindings = bindgen::Builder::default()
        .header(headers_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
