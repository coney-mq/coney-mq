#[derive(Debug)]
pub struct ExchangeDeclareRq {}

#[derive(Debug)]
pub struct ExchangeDeclareOk {}

#[derive(Debug, ::thiserror::Error)]
pub enum ExchangeDeclareErr {}

#[async_trait::async_trait]
pub trait ExchangeDeclare {
    async fn exchange_declare(
        &self,
        rq: ExchangeDeclareRq,
    ) -> Result<ExchangeDeclareOk, ExchangeDeclareErr>;
}
