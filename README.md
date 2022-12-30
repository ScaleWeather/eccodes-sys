# eccodes-sys

[![License](https://img.shields.io/github/license/ScaleWeather/eccodes-sys)](https://choosealicense.com/licenses/apache-2.0/)
[![Crates.io](https://img.shields.io/crates/v/eccodes-sys)](https://crates.io/crates/eccodes-sys)
[![dependency status](https://deps.rs/repo/github/ScaleWeather/eccodes-sys/status.svg)](https://deps.rs/repo/github/ScaleWeather/eccodes-sys)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ScaleWeather/eccodes-sys/rust.yml?branch=main&label=cargo%20build)](https://github.com/ScaleWeather/eccodes-sys/actions)

**This is a `-sys` crate with raw, unsafe bindings to the library and its API should not be used directly.** See the [eccodes crate](https://github.com/ScaleWeather/eccodes) for high-level, safe bindings.

**Due to the complexity of ecCodes library the decision has been made that this crate will not build ecCodes from source.**
See sections below for additional information how to install ecCodes on your system.

[ecCodes](https://confluence.ecmwf.int/display/ECC/ecCodes+Home) is an open-source library for reading and writing GRIB and BUFR files developed by [European Centre for Medium-Range Weather Forecasts](https://www.ecmwf.int/).

## Usage

This crate will look for existing `libeccodes` installation using [pkg-config](https://crates.io/crates/pkg-config).
The ecCodes library is then linked and bindings are generated using [bindgen](https://crates.io/crates/bindgen).
If the library is not found, the build will fail.

## ecCodes installation

The recommended way to install ecCodes on your computer is using your package manager.
For example, on Ubuntu you can use `apt-get`:

```bash
sudo apt-get install libeccodes-dev
```

Alternatively, you can install the library manually from source in suitable directory
following [this instructions](https://confluence.ecmwf.int/display/ECC/ecCodes+installation).

Then add the `lib/pkgconfig` directory from your ecCodes installation directory
to the `PKG_CONFIG_PATH` environmental variable. If ecCodes have been compiled
as shared library you will also need to specify `LD_LIBRARY_PATH`.
For example:

```bash
export PKG_CONFIG_PATH=<your_eccodes_path>/lib/pkgconfig
export LD_LIBRARY_PATH=<your_eccodes_path>/lib
```

## Features

There are two development features available:

- `docs` - for documentation building, does not link ecCodes and includes `bindings-docs.rs` into `lib.rs`
- `tests` - turns on generation of layout tests by `bindgen`, should not be used in production. Layout tests are off by default as they dereference null pointers causing undefined behavior

## License

The ecCodes library and these bindings are licensed under the [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
