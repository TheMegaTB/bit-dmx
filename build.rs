extern crate gcc;

use std::fs::*;
use std::path::{Path, PathBuf};

use std::env;

fn recursive_ls(dir: &Path) -> Vec<PathBuf> {
    let paths = read_dir(dir).unwrap();
    paths.flat_map(|path| {
        let path = path.unwrap().path();
        if metadata(path.clone()).unwrap().is_dir() { recursive_ls(&path) } else { vec![path] }
    }).collect::<Vec<_>>()
}

fn main() {
    // // Tell cargo not to recompile if any of the configs has changed
    // for if_c_source in recursive_ls(&Path::new("src/interface/")) {
    //     println!("cargo:rerun-if-changed={}", Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(if_c_source).display());
    // }
    //
    // // Compile the interface code
    // gcc::compile_library("libinterface.a", &["src/interface/arduino-serial-lib.c", "src/interface/arduino-serial-dmx.c"])
}
