extern crate gcc;

use std::process::Command;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut hash = if cfg!( any(unix) ) {
        Command::new("/usr/bin/git").arg("rev-parse").arg("--short").arg("HEAD").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        }).stdout
    } else { panic!("You shall not pas...ehm..compile on a non-unix OS!") };

    hash.pop();
    let mut f = File::create("src/structs/git_hash.rs").unwrap();
    f.write_all("pub const GIT_HASH: &'static str = \"".to_string().as_bytes()).unwrap();
    f.write_all(hash.as_slice()).unwrap();
    f.write_all("\";".to_string().as_bytes()).unwrap();

    gcc::compile_library("libinterface.a", &["src/interface/arduino-serial-lib.c", "src/interface/arduino-serial-dmx.c"])
}
