use super::*;

use ::amq_protocol::frame::ProtocolVersion;

use ::mq::vhost::VHost;

pub struct State {
    pub protocol_version: ProtocolVersion,
    pub identity: String,
    pub tuning: handshake::Tuning,
    pub vhost_name: String,
    pub vhost_api: Arc<dyn VHost>,
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<State>())
            .field("protocol_version", &self.protocol_version)
            .field("identity", &self.identity)
            .field("tuning", &self.tuning)
            .field("vhost_name", &self.vhost_name)
            .finish()
    }
}
