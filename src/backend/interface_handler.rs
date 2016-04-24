#![allow(dead_code)]
use std::time::Duration;
use std::thread::sleep;
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
    fn new() -> Interface {
        Interface {
            baudrate: 115200,
            port: "/dev/ttyACM0".to_string()
        }
    }

    fn baudrate(mut self, baudrate: usize) -> Interface {
        self.baudrate = baudrate;
        self
    }

    fn port(mut self, port: String) -> Interface {
        self.port = port;
        self
    }

    fn connect(self) -> Result<InterfaceHandle, &'static str> {
        match connect(self.baudrate, self.port.clone()) {
            true => Ok(InterfaceHandle {
                values: Vec::new(),
                interface: self
            }),
            false => Err("Couldn't connect.")
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
        thread::spawn(move|| {

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
                                insert_to_vector(&mut cache, rx.recv().unwrap());
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
        });
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

pub fn connect_and_test() {
    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, interrupt_tx) = interface.unwrap().to_thread();
    (0..255).chain((0..255).rev()).map(|i| {
        tx.send((1, i)).unwrap();
        tx.send((2, i)).unwrap();
        //tx.send((3, i)).unwrap();
        sleep(Duration::from_millis(16));
        // interface.write_to_dmx(1, i);
        // println!("{}", i);
    }).collect::<Vec<_>>();
    sleep(Duration::from_millis(2000));
    println!("Disconnecting...");
    interrupt_tx.send(true).unwrap();
}
