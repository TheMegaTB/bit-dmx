
/// Module for simple channel group to control a single channel
pub mod single;
/// Module for simple channel group to control rgb lights
pub mod rgb;
/// Module for simple channel group to control rgba lights
pub mod rgba;
/// Module for the channel group to control all these awesome moving heads
pub mod moving_2d;

pub use logic::channel_group::single::Single;
pub use logic::channel_group::rgb::RGB;
pub use logic::channel_group::rgba::RGBA;
pub use logic::channel_group::moving_2d::Moving2D;

#[derive(Debug)]
/// Enum to represent all channel group types that are supportet by bit dmx
pub enum ChannelGroup {
    /// Simple channel group to control a single channel
    Single(Single),
    /// Simple channel group to control rgb lights
    RGB(RGB),
    /// Simple channel group to control rgba lights
    RGBA(RGBA),
    /// The channel group to control all these awesome moving heads
    Moving2D(Moving2D)
}
