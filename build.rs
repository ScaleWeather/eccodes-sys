use std::{
    collections::HashSet,
    env,
    fs::File,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use flate2::read::GzDecoder;
use tar::Archive;

// build script heavily inspired by proj-sys crate
// some parts of code from rust-bindgen

const MINIMUM_ECCODES_VERSION: &str = "2.20.0"; // currently the latest in apt-get
const PROBLEMATIC_MACROS: [&str; 5] = [
    "FP_NAN",
    "FP_INFINITE",
    "FP_ZERO",
    "FP_SUBNORMAL",
    "FP_NORMAL",
];

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if PROBLEMATIC_MACROS.contains(&name) {
            return MacroParsingBehavior::Ignore;
        }

        MacroParsingBehavior::Default
    }
}

fn main() {
    let include_path;
    if cfg!(feature = "build_source") {
        include_path = get_include_from_source();
    } else {
        let lib_result = pkg_config::Config::new()
            .atleast_version(MINIMUM_ECCODES_VERSION)
            .probe("eccodes");

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

    //bindgen magic to avoid duplicate math.h type definitions
    let macros = Arc::new(RwLock::new(HashSet::new()));

    let tests = cfg!(feature = "tests");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.to_string_lossy()))
        .trust_clang_mangling(false)
        .header("wrapper.h")
        .layout_tests(tests) //avoiding test with UB
        .parse_callbacks(Box::new(MacroCallback {
            macros: macros.clone(),
        }))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings to file");
}

fn get_include_from_source() -> PathBuf {
    eprintln!("Building ecCodes from source so using specified features");

    //unpack archive
    let path = "eccodes-src/eccodes-2.22.1-Source.tar.gz";
    let tar_gz = File::open(path).expect("Failed to open ecCodes source archive");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive
        .unpack("eccodes-src")
        .expect("Failed to unpack ecCodes source archive");

    //build source with cmake
    let mut cmake_config = cmake::Config::new("eccodes-src/eccodes-2.22.1-Source");

    //default configuration
    cmake_config.define("CMAKE_C_FLAGS", "-O3");
    cmake_config.define("ENABLE_NETCDF", "OFF");
    cmake_config.define("ENABLE_FORTRAN", "OFF");
    cmake_config.define("ENABLE_JPG", "OFF");
    cmake_config.define("ENABLE_PNG", "OFF");
    cmake_config.define("ENABLE_MEMFS", "ON");
    cmake_config.define("BUILD_SHARED_LIBS", "OFF");

    if cfg!(feature = "shared") {
        cmake_config.define("BUILD_SHARED_LIBS", "ON");
    }

    if cfg!(feature = "no_memfs") {
        cmake_config.define("ENABLE_MEMFS", "OFF");
    }

    if cfg!(feature = "netcdf") {
        cmake_config.define("ENABLE_NETCDF", "ON");
    }

    if cfg!(feature = "jpeg") {
        cmake_config.define("ENABLE_JPG", "ON");
    }
    if cfg!(feature = "png") {
        cmake_config.define("ENABLE_PNG", "ON");
    }

    //path to built libeccodes
    let eccodes = cmake_config.build();

    //link the library
    if cfg!(feature = "shared") {
        println!("cargo:rustc-link-lib=eccodes");
    } else {
        println!("cargo:rustc-link-lib=static=eccodes");
    }

    println!(
        "cargo:rustc-link-search=native={}",
        eccodes.join("lib").display()
    );

    eccodes.join("include")
}
