//! Module containing all modules that provide the logic components behind the BitDMX software
/// Module for channel groups.
pub mod channel_group;
/// Module for channel group values.
pub mod channel_group_value;
/// Module for channels.
pub mod channel;
/// Module for chasers.
pub mod chaser;
/// Module for fades.
pub mod fade;
/// Module for fixtures.
pub mod fixture;
/// Module for stages.
pub mod stage;
/// Module for swiches.
pub mod switch;
/// Module for backend servers.
pub mod server;

pub use logic::channel_group::ChannelGroup;
pub use logic::channel_group_value::ChannelGroupValue;
pub use logic::channel::Channel;
pub use logic::chaser::Chaser;
pub use logic::fixture::Fixture;
pub use logic::stage::Stage;
pub use logic::switch::Switch;
