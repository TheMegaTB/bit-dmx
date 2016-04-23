use std::time::Duration;
use std::thread::sleep;

use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn open_port(baudrate: usize, port: *const c_char) -> bool;
    fn close_port();
    fn write_dmx(channel: u16, value: u8);
}

pub fn connect_and_test() {
    let port = CString::new("/dev/ttyACM0").unwrap();
    unsafe {
        if open_port(115200, port.as_ptr()) {
            println!("Connection established!");
            write_dmx(1, 255);
            sleep(Duration::from_millis(5000));
            write_dmx(1, 0);
            println!("Data transmitted.");
            close_port();
        } else {
            println!("Failed to open serial port!");
        }
    }
}
