use super::*;

#[async_trait::async_trait]
impl Handler<Open, ConnContext, LoopControl> for RegularChannel {
    async fn handle(
        &mut self,
        context: &mut ConnContext,
        query: Open,
    ) -> Result<LoopControl, AmqpException> {
        match self.chan_state {
            ChanState::Closed => {
                log::trace!("Openning channel#{}", self.id);
                self.chan_state = ChanState::Open;

                let open_ok = OpenOk {};
                let open_ok = AMQPMethod::OpenOk(open_ok);
                let open_ok = AMQPClass::Channel(open_ok);
                let open_ok = AMQPFrame::Method(self.id, open_ok);

                let () = context.send_frame(open_ok).await?;

                Ok(LoopControl::Continue)
            },
            _ => Err(AmqpException::new(format!(
                "Unexpected attempt to open channel#{}: {:?}",
                self.id, query
            ))
            .with_condition(Condition::ChannelError)),
        }
    }
}
