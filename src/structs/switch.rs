use std::collections::HashMap;

use ChannelGroupValue;

#[derive(Debug)]
pub struct Switch {
    pub channel_groups: HashMap<(usize, usize), ChannelGroupValue>,
    pub switch_group: usize
}

impl Switch {
    pub fn new(channel_groups: HashMap<(usize, usize), ChannelGroupValue>, switch_group: usize) -> Switch {
        Switch {
            channel_groups: channel_groups,
            switch_group: switch_group
        }
    }
}
