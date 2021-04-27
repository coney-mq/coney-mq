use super::*;

mod open_channel;
pub use open_channel::OpenChannel;

#[derive(Debug)]
pub enum RegularChannel {
    Closed,
    Open(OpenChannel),
}
