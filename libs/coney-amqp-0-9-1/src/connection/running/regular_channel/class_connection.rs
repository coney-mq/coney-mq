use super::*;

use ::amq_protocol::protocol::connection::AMQPMethod;

#[async_trait::async_trait]
impl Handler<AMQPMethod> for RegularChannel {
    async fn handle(
        &mut self,
        _context: &mut ConnContext,
        _query: AMQPMethod,
    ) -> Result<LoopControl, AmqpException> {
        Err(AmqpException::new(
            "Connection-class methods are supposed to be sent to the Control Channel",
        )
        .with_condition(Condition::CommandInvalid))
    }
}
