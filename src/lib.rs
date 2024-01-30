#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//!# Low-level Rust bindings for ecCodes
//!
//!**This is a `-sys` crate with raw, unsafe bindings to the library and its API should not be used directly.** 
//!See the [eccodes crate](https://github.com/ScaleWeather/eccodes) for high-level, safe bindings.
//!
//!**Due to the complexity of ecCodes library the decision has been made that this crate will not build ecCodes from source.**
//!See sections below for additional information how to install ecCodes on your system.
//!
//![ecCodes](https://confluence.ecmwf.int/display/ECC/ecCodes+Home) is an open-source library for 
//!reading and writing GRIB and BUFR files developed by [European Centre for Medium-Range Weather Forecasts](https://www.ecmwf.int/).
//!
//!## Usage
//!
//!This crate will look for existing `libeccodes` installation using [pkg-config](https://crates.io/crates/pkg-config).
//!The ecCodes library is then linked and bindings are generated using [bindgen](https://crates.io/crates/bindgen). 
//!If the library is not found, the build will fail. 
//!
//!## ecCodes installation
//!
//!The reccomended way to install ecCodes on your computer is using your package manager.
//!For example, on Ubuntu you can use `apt-get`:
//!
//!```text
//!$ sudo apt-get install libeccodes-dev
//!```
//!
//!Alternatively, you can install the library manually from source in suitable directory
//!following [this instructions](https://confluence.ecmwf.int/display/ECC/ecCodes+installation).
//!
//!Then add the `lib/pkgconfig` directory from your ecCodes installation directory
//!to the `PKG_CONFIG_PATH` environmental variable. For example:
//!
//!```text
//!$ export PKG_CONFIG_PATH=<your_eccodes_path>/lib/pkgconfig
//!```
//!
//!## Features
//!
//!There are two development features available:
//!
//!- `docs` - for documentation building, does not link ecCodes and includes `bindings-docs.rs` into `lib.rs`
//!- `tests` - turns on generation of layout tests by `bindgen`, should not be used in production. Layout tests are off by default as they derefrence null pointers causing undefined behaviour
//!

use std::sync::Mutex;

/// Global mutex to synchronize functions that fail in concurrent context,
/// eg. `codes_handle_new_from_file`, `codes_index_add_file` etc.
pub static CODES_LOCK: Mutex<()> = Mutex::new(());

#[cfg(not(feature = "docs"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "docs")]
include!("bindings-docs.rs");
