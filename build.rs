extern crate gcc;
extern crate find_folder;
extern crate flate2;
extern crate rustc_serialize;

use std::process::Command;
use std::io::prelude::*;
use std::fs::*;
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::collections::HashMap;
use rustc_serialize::json;

fn recursive_ls(dir: &Path) -> Vec<PathBuf> {
    let paths = read_dir(dir).unwrap();
    paths.flat_map(|path| {
        let path = path.unwrap().path();
        if metadata(path.clone()).unwrap().is_dir() { recursive_ls(&path) } else { vec![path] }
    }).collect::<Vec<_>>()
}

fn compress_folder(folder: &'static str) -> Vec<u8> {
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder(folder).unwrap();
    let paths = recursive_ls(&assets);
    let mut files = HashMap::new();
    for path in paths {
        let rel_path = format!("{:?}", Path::new(folder).join(path.as_path().strip_prefix(&assets).unwrap())).replace("\"", "");
        let mut data = Vec::new();
        let mut f = File::open(path).unwrap();
        f.read_to_end(&mut data).unwrap();
        files.insert(rel_path, data);
    }

    let mut json_map = json::encode(&files).unwrap().into_bytes();
    let mut e = ZlibEncoder::new(Vec::new(), Compression::Best);
    e.write_all(&mut json_map).unwrap();
    e.finish().unwrap()
}

fn compress_and_save_folder(folder: &'static str) {
    let data = compress_folder("assets");
    File::create("src/structs/compressed_data/".to_string() + &folder.to_string() + &".bin".to_string()).unwrap().write_all(&data).unwrap();
}

fn main() {
    // Read and compress the assets folder into a binary blob included in the binary
    compress_and_save_folder("assets");

    // Add the git hash as constant
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

    // Compile the interface code
    gcc::compile_library("libinterface.a", &["src/interface/arduino-serial-lib.c", "src/interface/arduino-serial-dmx.c"])
}
