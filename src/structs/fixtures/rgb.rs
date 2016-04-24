use DmxChannel;
use DmxValue;
use FadeCurve;
use FadeTime;
use std::sync::mpsc;

#[derive(Debug)]
pub struct RGB {
    channel: DmxChannel,
    valuer: DmxValue,
    valueg: DmxValue,
    valueb: DmxValue,
    dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>
}

impl RGB {
    fn fade_rgb(&mut self, curve: FadeCurve, endr: DmxValue, endg: DmxValue, endb: DmxValue, time: FadeTime) {

    }
}
