use std::{
    collections::HashSet,
    env,
    fs::File,
    io::copy,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use flate2::read::GzDecoder;
use tar::Archive;
use tokio;

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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if cfg!(feature="docs") {
        return;
    }

    let include_path;
    if cfg!(feature = "build_source") {
        include_path = get_include_from_source().await;
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
                include_path = get_include_from_source().await;
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

async fn get_include_from_source() -> PathBuf {
    eprintln!("Building ecCodes from source so using specified features");

    //path constants
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source_url =
        "https://confluence.ecmwf.int/download/attachments/45757960/eccodes-2.23.0-Source.tar.gz";
    let source_tar = out_path.join("eccodes-2.23.0-Source.tar.gz");
    let source_path = out_path.join("eccodes-2.23.0-Source");

    //download the source code
    let source_content = reqwest::get(source_url)
        .await
        .expect("Failed to download ecCodes source code")
        .bytes()
        .await
        .expect("Failed to convert downloaded file");

    //save the source code
    let mut dest = File::create(&source_tar).expect("Failed to create file");
    copy(&mut source_content.as_ref(), &mut dest).expect("Failed to save donwloaded tar");

    //unpack archive
    let tar_gz = File::open(&source_tar).expect("Failed to open ecCodes source archive");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive
        .unpack(out_path)
        .expect("Failed to unpack ecCodes source archive");

    //build source with cmake
    let mut cmake_config = cmake::Config::new(source_path);

    //cmake build configuration
    cmake_config.define("CMAKE_C_FLAGS", "-O3");
    cmake_config.define("ENABLE_FORTRAN", "OFF");
    cmake_config.define("ENABLE_EXAMPLES", "OFF");
    cmake_config.define("ENABLE_TESTS", "OFF");
    cmake_config.define("ENABLE_BUILD_TOOLS", "OFF");

    let shared = cfg!(feature = "shared");
    let no_memfs = cfg!(feature = "no_memfs");
    let netcdf = cfg!(feature = "netcdf");
    let jpeg = cfg!(feature = "jpeg");
    let png = cfg!(feature = "png");
    let posix = cfg!(feature = "posix");
    let aec = cfg!(feature = "aec");

    cmake_config.define("BUILD_SHARED_LIBS", if shared { "ON" } else { "OFF" });
    cmake_config.define("ENABLE_MEMFS", if no_memfs { "OFF" } else { "ON" });
    cmake_config.define("ENABLE_NETCDF", if netcdf { "ON" } else { "OFF" });
    cmake_config.define("ENABLE_JPG", if jpeg { "ON" } else { "OFF" });
    cmake_config.define("ENABLE_PNG", if png { "ON" } else { "OFF" });
    cmake_config.define("ENABLE_ECCODES_THREADS", if posix { "ON" } else { "OFF" });
    cmake_config.define("ENABLE_AEC", if aec { "ON" } else { "OFF" });

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
