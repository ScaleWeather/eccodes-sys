# eccodes-sys

![Licence](https://img.shields.io/github/license/ScaleWeather/eccodes-sys?style=flat-square)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/ScaleWeather/eccodes-sys/cargo?label=cargo%20build&style=flat-square)

**This is a `-sys` crate with raw, unsafe bindings to the library and its API should not be used directly.** See the [eccodes crate](https://github.com/ScaleWeather/eccodes) for high-level, safe bindings.

[ecCodes](https://confluence.ecmwf.int/display/ECC/ecCodes+Home) is an open-source library for reading and writing GRIB and BUFR files developed by [European Centre for Medium-Range Weather Forecasts](https://www.ecmwf.int/).

## Usage

By default this crate will look for existing `libeccodes` installation using [pkg-config](https://crates.io/crates/pkg-config). If the library is not found, the crate will build ecCodes from source provided with package.

To install ecCodes globally to use with this crate `apt-get` can be used:

```bash
sudo apt-get install libeccodes-dev
```

Because ecCodes supports mainly Linux platforms, this crate is not tested on other architectures.

### Features

The `eccodes-sys` crate allows to choose several features. For a detailed explanation of ecCodes compilation options check the [official website](https://confluence.ecmwf.int/display/ECC/ecCodes+installation).

- `build_source` - ecCodes library will be built from source even if other installation exists. This option by default builds static library with [MEMFS](https://confluence.ecmwf.int/pages/viewpage.action?pageId=143037711) activated.

All following features activate building from source:

- `jpeg` - builds ecCodes with JPEG2000 support enabled (`ENABLE_JPG=ON`). Requires `libopenjp2-7-dev` to be installed.
- `png` - builds ecCodes with PNG support enabled (`ENABLE_PNG=ON`). Requires `libpng-dev` to be installed.
- `netcdf` - builds ecCodes with NETCDF support enabled (`ENABLE_NETCDF=ON`). Requires `libnetcdff-dev` and `libnetcdf-c++4-dev` to be installed.

There are also two advanced features which should be used only when you know that compilation files will not be removed:

- `shared` - builds ecCodes as shared library (`BUILD_SHARED_LIBS=ON`)
- `no_memfs` - builds ecCodes with MEMFS deactivated (`ENABLE_MEMFS=OFF`)

## License

The ecCodes library and these bindings are licensed under the [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
