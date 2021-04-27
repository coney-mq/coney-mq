use ::amq_protocol::frame::AMQPFrame;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Props {
    pub channel_id: u16,
    pub class_id: u16,
    pub method_id: u16,
}

impl std::fmt::Display for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}|{}:{}]",
            self.channel_id, self.class_id, self.method_id
        )
    }
}

impl From<&AMQPFrame> for Props {
    fn from(v: &AMQPFrame) -> Self {
        match v {
            AMQPFrame::ProtocolHeader(_) => Default::default(),
            AMQPFrame::Method(channel_id, class) => Self {
                channel_id: *channel_id,
                class_id: class.get_amqp_class_id(),
                method_id: class.get_amqp_method_id(),
            },
            AMQPFrame::Header(channel_id, class_id, _) => Self {
                channel_id: *channel_id,
                class_id: *class_id,
                ..Default::default()
            },
            AMQPFrame::Body(channel_id, _) => Self {
                channel_id: *channel_id,
                ..Default::default()
            },
            AMQPFrame::Heartbeat(channel_id) => Self {
                channel_id: *channel_id,
                ..Default::default()
            },
        }
    }
}
