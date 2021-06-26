#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//!# Low-level Rust bindings for ecCodes
//!
//!**This is a `-sys` crate with raw, unsafe bindings to the library and its API should not be used directly.** See the [eccodes crate](https://github.com/ScaleWeather/eccodes) for high-level, safe bindings.
//!
//![ecCodes](https://confluence.ecmwf.int/display/ECC/ecCodes+Home) is an open-source library for reading and writing GRIB and BUFR files developed by [European Centre for Medium-Range Weather Forecasts](https://www.ecmwf.int/).
//!
//!## Usage
//!
//!By default this crate will look for existing `libeccodes` installation using [pkg-config](https://crates.io/crates/pkg-config). If the library is not found, the crate will attempt to download the latest source code and build ecCodes from source. The ecCodes library is then linked and bindings are generated using [bindgen](https://crates.io/crates/bindgen).
//!
//!### Features
//!
//!The `eccodes-sys` crate allows to choose several features. For a detailed explanation of ecCodes compilation options check the [official website](https://confluence.ecmwf.int/display/ECC/ecCodes+installation).
//!
//!- `build_source` - ecCodes library will be built from source even if other installation exists. This option by default builds static library with [MEMFS](https://confluence.ecmwf.int/pages/viewpage.action?pageId=143037711) activated.
//!
//!All following features activate building from source:
//!
//!- `jpeg` - builds ecCodes with JPEG2000 support enabled (`ENABLE_JPG=ON`), requires `libopenjp2-7-dev`
//!- `png` - builds ecCodes with PNG support enabled (`ENABLE_PNG=ON`), requires `libpng-dev`
//!- `netcdf` - builds ecCodes with NetCDF support enabled (`ENABLE_NETCDF=ON`), requires `libnetcdff-dev` and `libnetcdf-c++4-dev`
//!- `aec` - builds ecCodes with Adaptive Entropy Coding for decoding/encoding (CCSDS) enabled (`ENABLE_AEC=ON`), requires `libaec-dev`
//!- `posix` - builds ecCodes with POSIX threads support enabled (`ENABLE_ECCODES_THREADS=ON`)
//!
//!These are advanced features which should be used only when you know that compilation files will not be removed:
//!
//!- `shared` - builds ecCodes as shared library (`BUILD_SHARED_LIBS=ON`)
//!- `no_memfs` - builds ecCodes with MEMFS deactivated (`ENABLE_MEMFS=OFF`)
//!
//!There are also development features available:
//!
//!- `docs` - for documentation building, does not build or link ecCodes and includes `bindings-docs.rs` into `lib.rs`
//!- `tests` - turns on generation of layout tests by `bindgen`, should not be used in production. Layout tests are off by default as they derefrence null pointers causing undefined behaviour

#[cfg(not(feature = "docs"))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "docs")]
include!("bindings-docs.rs");
