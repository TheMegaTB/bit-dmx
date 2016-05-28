pub fn get_switch_update(shift_state: bool, addr: u16, value: u8) -> Vec<u8> {
    let addr_high = (addr >> 8) as u8;
    let addr_low = addr as u8;
    vec![if shift_state {129} else {1}, addr_high, addr_low, value]
}

pub fn get_start_chaser(shift_state: bool, addr: u16, value: u8) -> Vec<u8> {
    let addr_high = (addr >> 8) as u8;
    let addr_low = addr as u8;
    vec![if shift_state {130} else {2}, addr_high, addr_low, value]
}
