#[derive(Debug)]
pub struct ExchangeDeleteRq {}

#[derive(Debug)]
pub struct ExchangeDeleteOk {}

#[derive(Debug, ::thiserror::Error)]
pub enum ExchangeDeleteErr {}

#[async_trait::async_trait]
pub trait ExchangeDelete {
    async fn exchange_delete(
        &self,
        rq: ExchangeDeleteRq,
    ) -> Result<ExchangeDeleteOk, ExchangeDeleteErr>;
}
