use super::*;

#[derive(Debug)]
pub(super) struct RegularChannel {
    id: u16,
}

impl RegularChannel {
    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub async fn process_inbound_frame(
        &mut self,
        inbound_frame: AMQPFrame,
    ) -> Result<LoopControl, AmqpException> {
        Err(AmqpException::new("Not implemented")
            .with_condition(Condition::NotImplemented)
            .with_props(From::from(&inbound_frame)))
    }
}
