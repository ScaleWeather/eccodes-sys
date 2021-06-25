use std::{path::PathBuf, str::FromStr};

/*
if cfg(build-source)
    build_source()
else nothing
    search with pkg-config
    if failed
        build_source()

build_source() {
    if cfg(shared)
        BUILD_SHARED_LIBS=ON
    else
        BUILD_SHARED_LIBS=OFF

    if config(no-memfs)
        ENABLE_MEMFS=OFF
    else
        ENABLE_MEMFS=ON
}
 */

// build script heavily inspired by proj-sys crate

const MINIMUM_ECCODES_VERSION: &str = "2.20.0"; // currently the latest in apt-get

fn main() {
    let include_path;
    if cfg!(feature = "build_source") {
        include_path = get_include_from_source();
    } else {
        let lib_result = pkg_config::Config::new()
            .atleast_version(MINIMUM_ECCODES_VERSION)
            .probe("eccodes-dev");

        match lib_result {
            Ok(pk) => {
                eprintln!(
                    "Found installed ecCodes library to link at: {:?}",
                    pk.link_paths[0]
                );
                println!("cargo:rustc-link-search=native={:?}", pk.link_paths[0]);
                println!("cargo:rustc-link-lib=eccodes");
                include_path = pk.include_paths[0].clone();
            }
            Err(err) => {
                eprintln!("Cannot find existing ecCodes library {}", err);
                include_path = get_include_from_source();
            }
        }
    }

    let bindings = bindgen::Builder::default()
    .clang_arg(format!("-I{}", include_path.to_string_lossy()))
    .trust_clang_mangling(false)
    .header("wrapper.h")
    .raw_line("#![allow(non_upper_case_globals)]")
    .raw_line("#![allow(non_camel_case_types)]")
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(unused)]")
    .generate()
    .expect("Unable to generate bindings");

    bindings.write_to_file("src/bindings.rs").expect("Failed to write bidnings to file");
}

fn get_include_from_source() -> PathBuf {
    eprintln!("Building ecCodes from source so using specified features");

    let path = PathBuf::from_str("eccodes-src/include/eccodes.h").unwrap();

    path
}
