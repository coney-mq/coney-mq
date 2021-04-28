use super::*;

mod chan_state;
use chan_state::ChanState;

mod handler;
use handler::Handler;

mod class_channel;
mod class_connection;

#[derive(Debug)]
pub(super) struct RegularChannel {
    id: u16,
    chan_state: ChanState,
}

impl RegularChannel {
    pub fn new(id: u16) -> Self {
        let chan_state = ChanState::Closed;
        Self { id, chan_state }
    }

    pub async fn process_inbound_frame(
        &mut self,
        context: &mut ConnContext,
        inbound_frame: AMQPFrame,
    ) -> Result<LoopControl, AmqpException> {
        match inbound_frame {
            AMQPFrame::Method(channel_id, amqp_class) => {
                assert!(self.id == channel_id);
                match amqp_class {
                    AMQPClass::Connection(inner) => self.handle(context, inner).await,
                    AMQPClass::Channel(inner) => self.handle(context, inner).await,
                    _ => Err(AmqpException::new("Not implemented")
                        .with_condition(Condition::NotImplemented)),
                }
            }

            _ => {
                Err(AmqpException::new("Not implemented").with_condition(Condition::NotImplemented))
            }
        }
    }
}
