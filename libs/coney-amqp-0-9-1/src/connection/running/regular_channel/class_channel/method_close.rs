use super::*;

#[async_trait::async_trait]
impl Handler<Close, ConnContext, LoopControl> for RegularChannel {
    async fn handle(
        &mut self,
        context: &mut ConnContext,
        query: Close,
    ) -> Result<LoopControl, AmqpException> {
        match self.chan_state {
            ChanState::Open => {
                log::trace!("Closing channel#{}", self.id);
                self.chan_state = ChanState::Closed;

                let close_ok = CloseOk {};
                let close_ok = AMQPMethod::CloseOk(close_ok);
                let close_ok = AMQPClass::Channel(close_ok);
                let close_ok = AMQPFrame::Method(self.id, close_ok);

                let () = context.send_frame(close_ok).await?;

                Ok(LoopControl::Continue)
            },
            _ => Err(AmqpException::new(format!(
                "Unexpected attempt to close channel#{}: {:?}",
                self.id, query
            ))
            .with_condition(Condition::ChannelError)),
        }
    }
}
