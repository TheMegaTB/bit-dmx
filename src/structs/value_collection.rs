use std::collections::HashMap;

use ChannelGroupValue;
use ChannelGroup;
use DmxValue;
use FadeTime;
use FadeCurve;

#[derive(Debug)]
pub struct ValueCollection {
    pub channel_groups: HashMap<(usize, usize), ChannelGroupValue>
}

impl ValueCollection {
    pub fn new(channel_groups: HashMap<(usize, usize), (Vec<DmxValue>, FadeCurve, FadeTime)>) -> ValueCollection {
        ValueCollection {
            channel_groups: channel_groups
        }
    }
}
