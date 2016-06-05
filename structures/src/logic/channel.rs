use std::sync::mpsc;
use std::cmp;

use std::error::Error;

/// The default type for dmx channel values
pub type DmxValue = u8;
/// The default type for dmx channel addresses
pub type DmxAddress = u16;

#[derive(Debug)]
/// The struct that represents a dmx channel
pub struct Channel {
    /// The current value of the channel
    current_value: DmxValue,
    /// The value that is set to the channel. If the preheat value is higher then this value the current_value is set to the preheat value
    pub value: DmxValue,

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
    fn update(&mut self) {
        if self.current_value != self.value {
            self.current_value = self.value;
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
