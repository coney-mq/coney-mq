#[async_trait::async_trait]
pub trait VHost: Send + Sync + 'static {}
