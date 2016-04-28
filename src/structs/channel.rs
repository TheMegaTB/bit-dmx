use DmxValue;
use DmxAddress;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Channel {
    pub value: DmxValue,
    current_value: DmxValue,
    pub preheat_value: DmxValue,
    pub preheat_state: bool,
    pub preheating: bool,
    address: DmxAddress,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Channel {
    pub fn new(address: DmxAddress, old_value: DmxValue, preheat_value: DmxValue, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Channel {
        dmx_tx.send((address, old_value)).unwrap();
        Channel {
            value: old_value,
            current_value: old_value,
            preheat_value: preheat_value,
            preheat_state: false,
            preheating: false,
            address: address,
            dmx_tx: dmx_tx
        }
    }
    pub fn get(&self) -> DmxValue {
        self.current_value
    }
    pub fn set(&mut self, value: DmxValue) {
        if value == self.current_value { return }
        if !self.preheating {
            self.value = value;
        }
        if self.preheat_state {
            if self.preheat_value > self.value {
                self.current_value = self.preheat_value;
            }
            else {
                self.current_value = value;
            }
        }
        else {
            self.current_value = value;
        }
        self.dmx_tx.send((self.address, self.current_value)).unwrap();

    }
    pub fn activate_preheat(&mut self) {
        self.preheat_state = true;
    }
    pub fn deactivate_preheat(&mut self) {
        self.preheat_state = false;
    }
}
