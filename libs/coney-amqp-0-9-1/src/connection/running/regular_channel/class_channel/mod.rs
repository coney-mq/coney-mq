use super::*;

use ::amq_protocol::protocol::channel::*;

mod method_close;
mod method_close_ok;
mod method_open;
// mod method_flow;
// mod method_flow_ok;

#[async_trait::async_trait]
impl Handler<AMQPMethod> for RegularChannel {
    async fn handle(
        &mut self,
        context: &mut ConnContext,
        query: AMQPMethod,
    ) -> Result<LoopControl, AmqpException> {
        match query {
            AMQPMethod::Open(inner) => self.handle(context, inner).await,
            AMQPMethod::OpenOk(_inner) => unimplemented!(),

            AMQPMethod::Flow(_inner) => unimplemented!(),
            AMQPMethod::FlowOk(_inner) => unimplemented!(),

            AMQPMethod::Close(inner) => self.handle(context, inner).await,
            AMQPMethod::CloseOk(inner) => self.handle(context, inner).await,
        }
    }
}
