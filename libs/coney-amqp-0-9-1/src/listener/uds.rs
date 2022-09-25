use super::*;

use std::io::Error as IoError;
use std::path::Path;

use ::futures::prelude::*;
use ::tokio::net::{UnixListener, UnixStream};

#[async_trait::async_trait]
impl Accept for UnixListener {
    type Conn = UnixStream;
    type Err = IoError;

    async fn accept(&mut self) -> Result<Self::Conn, Self::Err> {
        let (conn, _peer_addr) = Self::accept(self).await?;
        Ok(conn)
    }
}

impl<S> AmqpListener<UnixListener, S> {
    pub async fn bind_uds<P>(bind_path: P, sink: S) -> Result<Self, IoError>
    where
        P: AsRef<Path>,
        S: Sink<UnixStream>,
    {
        let uds_listener = UnixListener::bind(bind_path)?;
        let amqp_listener = Self { accept: uds_listener, sink };
        Ok(amqp_listener)
    }
}
