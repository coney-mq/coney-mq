use ::amq_protocol::frame::ProtocolVersion;

use ::authc::AuthcFailure;
use ::common::AnyError;

#[derive(Debug, ::thiserror::Error)]
pub enum ConnectionError {
    #[error("ConnectionError::UnsupportedProtocolVersion: {0}")]
    UnsupportedProtocolVersion(ProtocolVersion),

    #[error("ConnectionError::NoChannel: {}", _0)]
    NoChannel(u16),

    #[error("ConnectionError::PeerGone")]
    PeerGone,

    #[error("ConnectionError::IO")]
    IO(#[source] AnyError),

    #[error("ConnectionError::UnexpectedFrame: expectd: {0}; got: {1}")]
    UnexpectedFrame(String, String),

    #[error("ConnectionError::Authc")]
    Authc(#[source] AuthcFailure),

    #[error("ConnectionError::AuthcTooManyChallenges")]
    AuthcTooManyChallenges,

    #[error("ConnectionError::TuneNegotiationFailure: [ field: {field:?}, max: {max}, requested: {requested} ]")]
    TuneNegotiationFailure {
        field: String,
        max: u32,
        requested: u32,
    },

    #[error("ConnectionError::ISE")]
    ISE(#[source] AnyError),

    #[error("ConnectionError::NoSuchVHost: {}", _0)]
    NoSuchVHost(String),
}

impl ConnectionError {
    pub fn unexpected_frame(expected: &str, got: &str) -> Self {
        Self::UnexpectedFrame(expected.to_owned(), got.to_owned())
    }
    pub fn tune_negotiation_failure(field: &str, max: u32, requested: u32) -> Self {
        Self::TuneNegotiationFailure {
            field: field.to_owned(),
            max,
            requested,
        }
    }
}
