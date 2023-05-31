extern crate bindgen;
extern crate cmake;

#[cfg(any(target_os="linux", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {

    // Build the static library with CMake.
    let mut config = cmake::Config::new("rtaudio");
    config.define("BUILD_SHARED_LIBS", "OFF");
    config.define("RTAUDIO_BUILD_STATIC_LIBS", "ON");

    #[cfg(target_os = "linux")]
    {
        config.cxxflag("-lstdc++ ");

        println!("cargo:rustc-link-lib=dylib=stdc++");

        #[cfg(feature = "alsa")]
        {
            config.define("RTAUDIO_API_ALSA", "ON");

            match pkg_config::Config::new().statik(false).probe("alsa") {
                Err(pkg_config::Error::Failure { command, output }) => panic!(
                    "Pkg-config failed - usually this is because alsa development headers are not installed.\n\n\
                    For Fedora users:\n# dnf install alsa-lib-devel\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libasound2-dev\n\n\
                    pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
                Err(e) => panic!("{}", e),
                Ok(alsa_library) => {
                    for lib in alsa_library.libs {
                        println!("cargo:rustc-link-lib={}", lib);
                    }
                } 
            };
        }
        #[cfg(not(feature = "alsa"))]
        config.define("RTAUDIO_API_ALSA", "OFF");

        #[cfg(feature = "pulse")]
        {
            config.define("RTAUDIO_API_PULSE", "ON");

            match pkg_config::Config::new().statik(false).probe("libpulse-simple") {
                Err(pkg_config::Error::Failure { command, output }) => panic!(
                    "Pkg-config failed - usually this is because pulse development headers are not installed.\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libpulse-dev\n\n\
                    pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
                Err(e) => panic!("{}", e),
                Ok(pulse_library) => {
                    for lib in pulse_library.libs {
                        println!("cargo:rustc-link-lib={}", lib);
                    }
                } 
            };
        }
        #[cfg(not(feature = "pulse"))]
        config.define("RTAUDIO_API_PULSE", "OFF");

        #[cfg(feature = "jack_linux")]
        {
            config.define("RTAUDIO_API_JACK", "ON");

            match pkg_config::Config::new().statik(false).probe("jack") {
                Err(pkg_config::Error::Failure { command, output }) => panic!(
                    "Pkg-config failed - usually this is because jack development headers are not installed.\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libjack-dev\n\n\
                    pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
                Err(e) => panic!("{}", e),
                Ok(jack_library) => {
                    for lib in jack_library.libs {
                        println!("cargo:rustc-link-lib={}", lib);
                    }
                } 
            };
        }
        #[cfg(not(feature = "jack_linux"))]
        config.define("RTAUDIO_API_JACK", "OFF");
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");

        match pkg_config::Config::new().statik(false).probe("libossaudio") {
            Err(pkg_config::Error::Failure { command, output }) => panic!(
                "Pkg-config failed - usually this is because oss audio development headers are not installed.\n\n\
                pkg_config details:\n{}\n", pkg_config::Error::Failure { command, output }),
            Err(e) => panic!("{}", e),
            Ok(oss_library) => {
                for oss in oss_library.libs {
                    println!("cargo:rustc-link-lib={}", lib);
                }
            } 
        };

        #[cfg(feature = "oss")]
        config.define("RTAUDIO_API_OSS", "ON");
        #[cfg(not(feature = "oss"))]
        config.define("RTAUDIO_API_OSS", "OFF");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=dylib=c++");

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
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=user32");

        #[cfg(feature = "ds")]
        {
            config.define("RTAUDIO_API_DS", "ON");

            println!("cargo:rustc-link-lib=dsound");
        }
        #[cfg(not(feature = "ds"))]
        config.define("RTAUDIO_API_DS", "OFF");

        #[cfg(feature = "asio")]
        config.define("RTAUDIO_API_ASIO", "ON");
        #[cfg(not(feature = "asio"))]
        config.define("RTAUDIO_API_ASIO", "OFF");

        #[cfg(feature = "wasapi")]
        {
            config.define("RTAUDIO_API_WASAPI", "ON");

            println!("cargo:rustc-link-lib=ksuser");
            println!("cargo:rustc-link-lib=mfplat");
            println!("cargo:rustc-link-lib=mfuuid");
            println!("cargo:rustc-link-lib=wmcodecdspuuid");
        }
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

    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=static=rtaudio");
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=static=rtaudiod");
    }

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
