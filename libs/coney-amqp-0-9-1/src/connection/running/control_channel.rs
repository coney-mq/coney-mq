use super::*;

use ::amq_protocol::protocol::connection::{AMQPMethod as AmqpMethodConn, CloseOk as ConnCloseOk};

#[derive(Debug)]
pub(super) struct ControlChannel {}

impl ControlChannel {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn process_inbound_frame(
        &mut self,
        context: &mut ConnContext,
        inbound_frame: AMQPFrame,
    ) -> Result<LoopControl, AmqpException> {
        match inbound_frame {
            AMQPFrame::Method(_, amqp_class) =>
                self.process_inbound_amqp_class(context, amqp_class).await,

            _ => Err(AmqpException::new("Not implemented")
                .with_condition(Condition::NotImplemented)
                .with_props(From::from(&inbound_frame))),
        }
    }
    async fn process_inbound_amqp_class(
        &mut self,
        context: &mut ConnContext,
        amqp_class: AMQPClass,
    ) -> Result<LoopControl, AmqpException> {
        match amqp_class {
            AMQPClass::Connection(AmqpMethodConn::Close(close)) => {
                log::debug!("Peer requested connection-close: {:?}", close);
                let close_ok = ConnCloseOk {};
                let close_ok = AmqpMethodConn::CloseOk(close_ok);
                let close_ok = AMQPClass::Connection(close_ok);
                let close_ok = AMQPFrame::Method(CTL_CHANNEL_ID, close_ok);
                let _ = context.send_frame(close_ok).await;
                Ok(LoopControl::Break)
            },

            AMQPClass::Connection(unexpected) => {
                log::warn!("Received unexpected Connection-method: {:?}", unexpected);
                Err(AmqpException::new("Unexpected Connection-method")
                    .with_condition(Condition::CommandInvalid))
            },

            _ =>
                Err(AmqpException::new("Control channel only expects AMQP-Class Connection frames")
                    .with_condition(Condition::CommandInvalid)),
        }
    }
}
