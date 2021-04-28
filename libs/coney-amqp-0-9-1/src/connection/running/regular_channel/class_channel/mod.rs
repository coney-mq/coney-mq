use super::*;

use ::amq_protocol::protocol::channel::*;

mod method_close;
mod method_open;
// mod method_close_ok;
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
            AMQPMethod::OpenOk(inner) => unimplemented!(),

            AMQPMethod::Flow(inner) => unimplemented!(),
            AMQPMethod::FlowOk(inner) => unimplemented!(),

            AMQPMethod::Close(inner) => self.handle(context, inner).await,
            AMQPMethod::CloseOk(inner) => unimplemented!(),
        }
    }
}
