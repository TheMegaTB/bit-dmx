use DmxChannel;
use DmxValue;
use FadeCurve;
use FadeTime;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Single {
    channel: DmxChannel,
    value: DmxValue,
    dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>
}

impl Single {
    fn fade(&mut self, curve: FadeCurve, end: DmxValue, time: FadeTime) {

    }
}
