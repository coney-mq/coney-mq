use super::*;

#[derive(Debug, ::thiserror::Error)]
pub enum ConnectionError {
    #[error("ConnectionError::HandshakeError")]
    HandshakeError(#[source] handshake::HandshakeError),

    #[error("ConnectionError::IO")]
    IO(#[source] AnyError),
}
