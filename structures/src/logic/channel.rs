use std::sync::mpsc;
use std::cmp;

use std::error::Error;

pub type DmxValue = u8;
pub type DmxAddress = u16;

#[derive(Debug)]
pub struct Channel {
    current_value: DmxValue,

    pub value: DmxValue,
    pub preheat_value: DmxValue,
    pub max_preheat_value: DmxValue,

    pub address: DmxAddress,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>,
    pub current_thread: Option<mpsc::Sender<()>>
}

impl Channel {
    pub fn new(address: DmxAddress, old_value: DmxValue, max_preheat_value: DmxValue, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Channel {
        dmx_tx.send((address, old_value)).unwrap();
        Channel {
            current_value: old_value,
            value: old_value,
            preheat_value: 0,
            max_preheat_value: max_preheat_value,
            address: address,
            dmx_tx: dmx_tx,
            current_thread: None
        }
    }
    pub fn get(&self) -> DmxValue {
        self.current_value
    }
    pub fn set(&mut self, value: DmxValue) {
        self.value = value;
        self.update();
    }
    pub fn set_preheat(&mut self, value: DmxValue) {
        self.preheat_value = value;
        self.update();
    }
    fn update(&mut self) {
        let new_value = cmp::max(self.preheat_value, self.value);
        if self.current_value != new_value {
            self.current_value = new_value;
            match self.dmx_tx.send((self.address, self.current_value)) {
                Ok(_) => {}, Err(e) => {exit!(7, "Failed to send value to dmx interface handler: {}", e.description());}
            };
        }
    }
    pub fn stop_fade(&mut self) {
        match self.current_thread {
            Some(ref tx) => {
                if tx.send(()).is_ok() {trace!("Killed ongoing fade on channel {}", self.address)}
            },
            None => {}
        }
        self.current_thread = None;
    }
}
