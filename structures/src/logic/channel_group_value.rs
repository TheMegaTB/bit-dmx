use logic::channel::DmxValue;
use logic::fade::FadeCurve;
use logic::fade::FadeTime;

pub type ChannelGroupValueTuple = (Vec<DmxValue>, (FadeCurve, FadeTime), (FadeCurve, FadeTime));

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct ChannelGroupValue {
    pub values: Vec<DmxValue>,
    pub curve_in: FadeCurve,
    pub time_in: FadeTime,
    pub curve_out: FadeCurve,
    pub time_out: FadeTime
}

impl ChannelGroupValue {
    pub fn from_tuple(tuple: ChannelGroupValueTuple) -> ChannelGroupValue {
        ChannelGroupValue {
            values: tuple.0,
            curve_in:  (tuple.1).0,
            time_in: (tuple.1).1,
            curve_out: (tuple.2).0,
            time_out: (tuple.2).1
        }
    }
}
