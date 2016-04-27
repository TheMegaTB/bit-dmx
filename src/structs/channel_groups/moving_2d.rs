use DmxAddress;
use DmxValue;
use FadeCurve;
use FadeTime;
use std::sync::mpsc;
use get_fade_steps_int;
use FADE_TICKS;

use std::time::Duration;
use std::thread::sleep;

#[derive(Debug)]
pub struct Moving2D {
    channel: DmxAddress,
    x: DmxValue,
    y: DmxValue,
    dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>
}

impl Moving2D {
    pub fn new(channel: DmxAddress, dmx_tx: mpsc::Sender<(DmxAddress, DmxValue)>) -> Moving2D {
        Moving2D {
            channel: channel,
            x: 0,
            y: 0,
            dmx_tx: dmx_tx
        }
    }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_x: DmxValue, end_y: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        for (&x, &y) in get_fade_steps_int(self.x, end_x, steps, curve.clone()).iter().zip(get_fade_steps_int(self.y, end_y, steps, curve.clone()).iter()) {
            self.dmx_tx.send((self.channel + 0, x)).unwrap();
            self.dmx_tx.send((self.channel + 1, y)).unwrap();
            self.x = x;
            self.y = y;
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }
}
