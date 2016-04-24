extern crate structures;

mod interface_handler;

use interface_handler::*;
use structures::*;

fn main() {
    connect_and_test();
    let _stage = Stage::new();
}
