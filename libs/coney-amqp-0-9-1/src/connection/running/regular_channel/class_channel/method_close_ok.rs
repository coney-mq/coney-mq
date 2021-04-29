use super::*;

#[async_trait::async_trait]
impl Handler<CloseOk, ConnContext, LoopControl> for RegularChannel {
    async fn handle(
        &mut self,
        _context: &mut ConnContext,
        query: CloseOk,
    ) -> Result<LoopControl, AmqpException> {
        match self.chan_state {
            ChanState::Closing => {
                log::trace!("Closing channel#{}", self.id);
                self.chan_state = ChanState::Closed;
                Ok(LoopControl::Continue)
            }
            _ => Err(AmqpException::new(format!(
                "Unexpected close-ok came to to channel#{}: {:?}",
                self.id, query
            ))
            .with_condition(Condition::CommandInvalid)),
        }
    }
}
