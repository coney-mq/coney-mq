#[derive(Debug)]
pub struct ExchangeInspectRq {}

#[derive(Debug)]
pub struct ExchangeInspectOk {}

#[derive(Debug, ::thiserror::Error)]
pub enum ExchangeInspectErr {}

#[async_trait::async_trait]
pub trait ExchangeInspect {
    async fn exchange_inspect(
        &self,
        rq: ExchangeInspectRq,
    ) -> Result<ExchangeInspectOk, ExchangeInspectErr>;
}
