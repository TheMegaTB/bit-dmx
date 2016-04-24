use DmxChannel;
use DmxValue;
use FadeCurve;
use FadeTime;
use std::sync::mpsc;
use get_fade_steps;
use FADE_TICKS;

#[derive(Debug)]
pub struct Single {
    channel: DmxChannel,
    value: DmxValue,
    dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>
}

impl Single {
    fn fade(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue) {
        get_fade_steps(self.value, end_value, time*FADE_TICKS/1000, curve);
    }
}
