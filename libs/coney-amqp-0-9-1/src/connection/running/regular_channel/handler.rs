use super::*;

#[async_trait::async_trait]
pub(super) trait Handler<T, C, R>: Send + Sync + 'static {
    async fn handle(&mut self, context: &mut C, query: T) -> Result<R, AmqpException>;
}
