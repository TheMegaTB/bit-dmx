use Fixture;
use Scene;
use Switch;
use Channel;
use DmxAddress;
use DmxValue;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

#[derive(Debug)]
pub struct Stage {
    channels: Vec<Arc<Mutex<Channel>>>,
    fixtures: Vec<Fixture>,
    scenes: Vec<Scene>,
    switchs: Vec<Switch>,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Stage {
    pub fn new(dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Stage {
        Stage {
            channels: Vec::new(),
            fixtures: Vec::new(),
            scenes: Vec::new(),
            switchs: Vec::new(),
            dmx_tx: dmx_tx
        }
    }
    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    pub fn get_channel_object(&mut self, channel: DmxAddress) -> Arc<Mutex<Channel>> {
        for i in self.channels.len() as u16..channel as u16 {
            self.channels.push(Arc::new(Mutex::new(Channel::new(i + 1, 0, 10, self.dmx_tx.clone()))));
            println!("Create channel {}", i + 1);
        }
        self.channels[channel as usize - 1].clone()
    }
}
