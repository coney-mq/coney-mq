use super::*;

#[derive(Debug, ::thiserror::Error)]
pub enum HandshakeError {
    #[error("HandshakeError::ExpectedControlChannel [ch-id: {}]", channel_id)]
    ExpectedControlChannel { channel_id: u16 },

    #[error("HandshakeError::RecvError")]
    RecvError(#[source] util::RecvError),

    #[error("HandshakeERror::SendError")]
    SendError(#[source] AnyError),

    #[error("HandshakeError::UnexpectedFrame [exp: {}, act: {}]", expected, actual)]
    UnexpectedFrame {
        expected: &'static str,
        actual: String,
    },

    #[error("HandshakeError::UnsupportedProtocolVersion: {}", version)]
    UnsupportedProtocolVersion { version: ProtocolVersion },

    #[error("HandshakeError::AuthcTooManyChallenges")]
    AuthcTooManyChallenges,

    #[error("HandshakeError::AuthcMechError")]
    AuthcMechError(#[source] ::authc::AuthcFailure),

    #[error("HandshakeError::TuneNegotiationError")]
    TuneNegotiationError {
        field: &'static str,
        max: u32,
        requested: u32,
    },

    #[error("HandshakeError::NoSuchVHost: {}", _0)]
    NoSuchVHost(String),

    #[error("HandshakeError::ISE")]
    ISE(#[source] AnyError),
}

impl From<util::RecvError> for HandshakeError {
    fn from(v: util::RecvError) -> Self {
        Self::RecvError(v)
    }
}
impl From<::authc::AuthcFailure> for HandshakeError {
    fn from(v: ::authc::AuthcFailure) -> Self {
        Self::AuthcMechError(v)
    }
}
