use DmxValue;
use DmxAddress;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Channel {
    value: DmxValue,
    pub preheat_value: DmxValue,
    pub preheat_state: bool,
    address: DmxAddress,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Channel {
    pub fn new(address: DmxAddress, old_value: DmxValue, preheat_value: DmxValue, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Channel {
        dmx_tx.send((address, old_value)).unwrap();
        Channel {
            value: old_value,
            preheat_value: preheat_value,
            preheat_state: false,
            address: address,
            dmx_tx: dmx_tx
        }
    }
    pub fn get(&self) -> DmxValue {
        self.value
    }
    pub fn set(&mut self, value: DmxValue) {
        if (self.preheat_state) && (self.preheat_value > self.value) {
            self.value = self.preheat_value;
        }
        else {
            self.value = value;
        }
        self.dmx_tx.send((self.address, self.value)).unwrap();
    }
    pub fn activate_preheat(&mut self) {
        self.preheat_state = true;
    }
    pub fn deactivate_preheat(&mut self) {
        self.preheat_state = false;
    }
}
