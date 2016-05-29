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

// fn compress_file(file: &Path) -> Vec<u8> {
//     let mut f = File::open(file).unwrap();
//     let mut e = ZlibEncoder::new(Vec::new(), Compression::Default);
//     let mut buf = Vec::new();
//
//     f.read_to_end(&mut buf).unwrap();
//     e.write_all(&mut buf).unwrap();
//
//     e.finish().unwrap()
// }
//
// fn compress_folder_old(folder: &'static str) -> HashMap<String, String> {
//     let assets = find_folder::Search::KidsThenParents(3, 5)
//         .for_folder(folder).unwrap();
//     let paths = recursive_ls(&assets);
//
//     let mut compressed_files = HashMap::new();
//     for path in paths {
//         let rel_path = format!("{:?}", Path::new(folder).join(path.as_path().strip_prefix(&assets).unwrap())).replace("\"", "");
//         let data = json::encode(&compress_file(path.as_path())).unwrap();//format!("{:?}", compress_file(path.as_path()));
//         compressed_files.insert(rel_path, data);
//     }
//     compressed_files
// }

fn compress_folder(folder: &'static str) -> Vec<u8> {
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder(folder).unwrap();
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

fn main() {
    // Read and compress the assets folder into a constant
    let assets = compress_folder("assets");
    let mut f = File::create("src/structs/compressed_data/assets.bin").unwrap();
    f.write_all(&assets).unwrap();
    // std::process::exit(1);

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
