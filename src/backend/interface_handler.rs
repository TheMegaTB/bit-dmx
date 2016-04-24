#![allow(dead_code)]
use std::sync::mpsc;
use std::thread;

use std::ffi::CString;
use std::os::raw::c_char;

use structures::DmxChannel;
use structures::DmxValue;

extern {
    fn open_port(baudrate: usize, port: *const c_char) -> bool;
    fn close_port();
    fn write_dmx(channel: u16, value: u8);
    fn is_connected() -> bool;
    fn set_fake_interface_mode(enabled: bool);
}

fn connect(baudrate: usize, port: String) -> bool {
    unsafe {
        open_port(baudrate, CString::new(port).unwrap().as_ptr())
    }
}

fn disconnect() {
    unsafe { close_port() }
}

fn write_to_dmx(channel: DmxChannel, value: DmxValue) {
    unsafe { write_dmx(channel, value) }
}

pub struct Interface {
    baudrate: usize,
    port: String
}

pub struct InterfaceHandle {
    values: Vec<DmxValue>,
    interface: Interface
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            baudrate: 115200,
            port: "/dev/ttyACM0".to_string()
        }
    }

    pub fn baudrate(mut self, baudrate: usize) -> Interface {
        self.baudrate = baudrate;
        self
    }

    pub fn port(mut self, port: String) -> Interface {
        self.port = port;
        self
    }

    pub fn connect(self) -> Result<InterfaceHandle, &'static str> {
        if cfg!( feature = "fake_if" ) {
            unsafe { set_fake_interface_mode(true); }
            Ok(InterfaceHandle {
                values: Vec::new(),
                interface: self
            })
        } else {
            match connect(self.baudrate, self.port.clone()) {
                true => Ok(InterfaceHandle {
                    values: Vec::new(),
                    interface: self
                }),
                false => Err("Couldn't connect.")
            }
        }
    }
}

type DmxTouple = (DmxChannel, DmxValue);
fn insert_to_vector(cache: &mut Vec<DmxTouple>, elem: DmxTouple) {
    match cache.iter().position(|&x| x.0 == elem.0 ) {
        Some(index) => cache[index] = elem,
        None => cache.push(elem)
    }
}

impl InterfaceHandle {
    pub fn to_thread(self) -> (mpsc::Sender<(DmxChannel, DmxValue)>, mpsc::Sender<bool>) {
        let (tx, rx) = mpsc::channel();
        let (interrupt_tx, interrupt_rx) = mpsc::channel();
        thread::Builder::new().name("DMX-IF".to_string()).spawn(move|| {

            let mut cache: Vec<DmxTouple> = Vec::with_capacity(16);

            loop {
                if interrupt_rx.try_recv().is_ok() {
                    self.disconnect();
                    return;
                }

                let mut rx_available = true;
                while rx_available {
                    match rx.try_recv() {
                        Ok(elem) => {
                            insert_to_vector(&mut cache, elem);
                        },
                        Err(_) => {
                            if cache.len() == 0 {
                                insert_to_vector(&mut cache, rx.recv().unwrap()); //TODO: replace unwrap()
                            } else {
                                rx_available = false;
                            }
                        }
                    }
                }

                match cache.drain(0..1).last() {
                    Some(elem) => {
                        self.write_to_dmx(elem.0, elem.1);
                    },
                    None => {} //This shouldn't happen regardless.
                }
            }
        }).unwrap();
        (tx, interrupt_tx)
    }

    pub fn write_to_dmx(&self, channel: DmxChannel, value: DmxValue) {
        write_to_dmx(channel, value);
    }

    pub fn disconnect(self) -> Interface {
        disconnect();
        self.interface
    }
}
