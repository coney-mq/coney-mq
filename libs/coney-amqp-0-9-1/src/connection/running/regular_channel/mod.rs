use super::*;

use ::common::ErrorReport;

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
            },

            _ =>
                Err(AmqpException::new("Not implemented").with_condition(Condition::NotImplemented)),
        }
    }

    pub async fn process_channel_error(
        &mut self,
        context: &mut ConnContext,
        error: AmqpException,
    ) -> Result<LoopControl, AnyError> {
        let chan_state = std::mem::replace(&mut self.chan_state, ChanState::Invalid);
        let (next_state, loop_control) = match chan_state {
            ChanState::Open { .. } => {
                use ::amq_protocol::protocol::channel::{
                    AMQPMethod as AmqpMethodChan, Close as ChanClose,
                };

                let channel_id = self.id;
                let reply_code = error.condition().id();
                let reply_text = error.error_report();
                let class_id = error.props().class_id;
                let method_id = error.props().method_id;

                let pdu =
                    ChanClose { class_id, method_id, reply_code, reply_text: reply_text.into() };
                let close_rq = AmqpMethodChan::Close(pdu);
                let close_rq = AMQPClass::Channel(close_rq);
                let close_rq = AMQPFrame::Method(channel_id, close_rq);

                let () =
                    context.send_frame(close_rq).await.map_err(Into::into).map_err(ISE::Generic)?;

                (ChanState::Closing, LoopControl::Continue)
            },
            chan_state => {
                log::warn!(
                    "Unexpected amqp-exception while channel#{} is in state: {:?}:\n{}",
                    self.id,
                    chan_state,
                    error.error_report()
                );
                (chan_state, LoopControl::Continue)
            },
        };
        self.chan_state = next_state;
        Ok(loop_control)
    }
}
