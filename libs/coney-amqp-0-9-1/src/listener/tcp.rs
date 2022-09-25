use super::*;

use std::io::Error as IoError;
use std::net::SocketAddr;

use ::futures::prelude::*;
use ::tokio::net::{TcpListener, TcpStream};

#[async_trait::async_trait]
impl Accept for TcpListener {
    type Conn = TcpStream;
    type Err = IoError;

    async fn accept(&mut self) -> Result<Self::Conn, Self::Err> {
        let (conn, _peer_addr) = Self::accept(self).await?;
        Ok(conn)
    }
}

impl<S> AmqpListener<TcpListener, S> {
    pub async fn bind_tcp(bind_addr: SocketAddr, sink: S) -> Result<Self, IoError>
    where
        S: Sink<TcpStream>,
    {
        let tcp_listener = TcpListener::bind(bind_addr).await?;
        let amqp_listener = Self { accept: tcp_listener, sink };
        Ok(amqp_listener)
    }
}
