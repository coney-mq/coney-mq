use super::*;

use ::amq_protocol::frame::ProtocolVersion;

use ::mq::vhost::VHost;

use crate::channels::Channel;
use crate::channels::RegularChannel;

pub struct State {
    pub protocol_version: ProtocolVersion,
    pub identity: String,
    pub tuning: handshake::Tuning,
    pub vhost_name: String,
    pub vhost_api: Arc<dyn VHost>,
    pub channels: Vec<Channel>,
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<State>())
            .field("protocol_version", &self.protocol_version)
            .field("identity", &self.identity)
            .field("tuning", &self.tuning)
            .field("vhost_name", &self.vhost_name)
            .field("channels", &ChannelsFmtDebug(self.channels.as_ref()))
            .finish()
    }
}

struct ChannelsFmtDebug<'a>(&'a [Channel]);
impl<'a> std::fmt::Debug for ChannelsFmtDebug<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_map();
        for (chan_id, channel) in self.0.into_iter().enumerate() {
            match channel {
                Channel::Control(control) => {
                    let () = f.key(&chan_id).value(control).finish()?;
                }
                Channel::Regular(RegularChannel::Open(open)) => {
                    let () = f.key(&chan_id).value(open).finish()?;
                }
                _ => (),
            }
        }
        f.finish()
    }
}
