use ::common::AnyError;

use crate::amqp_framing::IoStream;

#[async_trait::async_trait]
pub trait Accept {
    type Conn: IoStream;
    type Err: Into<AnyError>;

    async fn accept(&mut self) -> Result<Self::Conn, Self::Err>;
}
