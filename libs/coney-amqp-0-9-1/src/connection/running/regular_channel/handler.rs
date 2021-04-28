use super::*;

#[async_trait::async_trait]
pub(super) trait Handler<T>: Send + Sync + 'static {
    async fn handle(
        &mut self,
        context: &mut ConnContext,
        query: T,
    ) -> Result<LoopControl, AmqpException>;
}
