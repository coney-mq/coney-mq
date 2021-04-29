pub mod exchange_declare;
pub mod exchange_delete;
pub mod exchange_inspect;

#[async_trait::async_trait]
pub trait VHost:
    Send
    + Sync
    + 'static
    + exchange_inspect::ExchangeInspect
    + exchange_declare::ExchangeDeclare
    + exchange_delete::ExchangeDelete
{
}

impl<T> VHost for T where
    T: Send
        + Sync
        + 'static
        + exchange_inspect::ExchangeInspect
        + exchange_declare::ExchangeDeclare
        + exchange_delete::ExchangeDelete
{
}
