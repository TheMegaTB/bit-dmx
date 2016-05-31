extern crate gcc;

fn main() {
    // Compile the interface code
    gcc::compile_library("libinterface.a", &["src/interface/arduino-serial-lib.c", "src/interface/arduino-serial-dmx.c"])
}
