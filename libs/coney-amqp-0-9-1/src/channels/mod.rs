mod control_channel;
pub use control_channel::ControlChannel;

mod regular_channel;
pub use regular_channel::OpenChannel;
pub use regular_channel::RegularChannel;

#[derive(Debug)]
pub enum Channel {
    Control(ControlChannel),
    Regular(RegularChannel),
}

impl Channel {
    pub fn create_control_channel() -> Self {
        Self::Control(ControlChannel {})
    }
    pub fn create_regular_channel() -> Self {
        Self::Regular(RegularChannel::Closed)
    }
}
