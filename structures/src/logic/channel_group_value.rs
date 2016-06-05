use logic::channel::DmxValue;
use logic::fade::FadeCurve;
use logic::fade::FadeTime;

/// The struct to save the ChannelGroupValue
pub type ChannelGroupValueTuple = (Vec<DmxValue>, (FadeCurve, FadeTime), (FadeCurve, FadeTime));

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
/// A tuple to save the values of a channel group including the target dmx values and the fade in and out curve and time
pub struct ChannelGroupValue {
    /// A Vector containing all the dmx values needed for the channel group
    pub values: Vec<DmxValue>,
    /// The curve used to fade in
    pub curve_in: FadeCurve,
    /// The time used to fade in
    pub time_in: FadeTime,
    /// The curve used to fade out
    pub curve_out: FadeCurve,
    /// The time used to fade out
    pub time_out: FadeTime
}

impl ChannelGroupValue {
    /// Create an ChannelGroupValue from an existing ChannelGroupValueTuple
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
