use std::sync::mpsc;

use std::error::Error;

/// The default type for dmx channel values
pub type DmxValue = u8;
/// The default type for dmx channel addresses
pub type DmxAddress = u16;

#[derive(Debug)]
/// The struct that represents a dmx channel
pub struct Channel {
    /// The current value of the channel
    value: DmxValue,
    /// The dmx address of the channel.
    pub address: DmxAddress,
    /// The sender to send channel informaton to the interface.
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>,
    /// The interrupt sender to interrupt a fade.
    pub current_thread: Option<mpsc::Sender<()>>
}

impl Channel {
    /// Generate a Channel from the given information.
    pub fn new(address: DmxAddress, old_value: DmxValue, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Channel {
        dmx_tx.send((address, old_value)).unwrap();
        Channel {
            value: old_value,
            address: address,
            dmx_tx: dmx_tx,
            current_thread: None
        }
    }
    /// Return the current value
    pub fn get(&self) -> DmxValue {
        self.value
    }
    /// Set the value of the channel
    pub fn set(&mut self, value: DmxValue) {
        if self.value != value {
            self.value = value;
            self.update();
        }
    }
    /// send an update request to the inerface, if the value has changed.
    fn update(&mut self) {
        match self.dmx_tx.send((self.address, self.value)) {
            Ok(_) => {}, Err(e) => {exit!(7, "Failed to send value to dmx interface handler: {}", e.description());}
        };
    }
    /// Stop the current fade if it exists
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
