use super::*;

#[derive(Debug, ::thiserror::Error)]
pub enum ConnectionError {
    #[error("ConnectionError::HandshakeError")]
    HandshakeError(#[source] handshake::HandshakeError),

    #[error("ConnectionError::IO")]
    IO(#[source] AnyError),

    #[error("ConnectionError::ISE")]
    ISE(#[source] ISE),

    #[error("ConnectionError::PeerGone")]
    PeerGone,
}

#[derive(Debug, ::thiserror::Error)]
pub enum ISE {
    #[error("ISE::Generic")]
    Generic(#[source] AnyError),

    #[error("ISE::SendQueueEndOfStream")]
    SendQueueEndOfStream,
}
