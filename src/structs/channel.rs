use std::sync::mpsc;
use std::cmp;

use DmxValue;
use DmxAddress;

#[derive(Debug)]
pub struct Channel {
    current_value: DmxValue,

    pub value: DmxValue,
    pub preheat_value: DmxValue,
    pub max_preheat_value: DmxValue,

    address: DmxAddress,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Channel {
    pub fn new(address: DmxAddress, old_value: DmxValue, preheat_value: DmxValue, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Channel {
        dmx_tx.send((address, old_value)).unwrap();
        Channel {
            current_value: old_value,
            value: old_value,
            preheat_value: preheat_value,
            max_preheat_value: preheat_value,
            address: address,
            dmx_tx: dmx_tx
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
        self.current_value = cmp::max(self.preheat_value, self.value);
        self.dmx_tx.send((self.address, self.current_value)).unwrap();
    }
}
