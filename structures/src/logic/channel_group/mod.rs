//TODO create a trait to do fixture.simple_fade

pub mod rgb;
pub mod rgba;
pub mod single;
pub mod moving_2d;

pub use logic::channel_group::single::Single;
pub use logic::channel_group::rgb::RGB;
pub use logic::channel_group::rgba::RGBA;
pub use logic::channel_group::moving_2d::Moving2D;

#[derive(Debug)]
pub enum ChannelGroup {
    Single(Single),
    RGB(RGB),
    RGBA(RGBA),
    Moving2D(Moving2D)
}
