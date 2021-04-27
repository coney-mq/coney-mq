use super::*;

#[derive(Debug, ::thiserror::Error)]
pub enum ConnectionError {
    // #[error("ConnectionError::UnsupportedProtocolVersion: {0}")]
    // UnsupportedProtocolVersion(ProtocolVersion),

    // #[error("ConnectionError::NoChannel: {}", _0)]
    // NoChannel(u16),

    // #[error("ConnectionError::PeerGone")]
    // PeerGone,
    #[error("ConnectionError::IO")]
    IO(#[source] AnyError),
    // #[error("ConnectionError::UnexpectedFrame: expectd: {0}; got: {1}")]
    // UnexpectedFrame(String, String),

    // #[error("ConnectionError::Authc")]
    // Authc(#[source] AuthcFailure),

    // #[error("ConnectionError::AuthcTooManyChallenges")]
    // AuthcTooManyChallenges,

    // #[error("ConnectionError::TuneNegotiationFailure: [ field: {field:?}, max: {max}, requested: {requested} ]")]
    // TuneNegotiationFailure {
    //     field: String,
    //     max: u32,
    //     requested: u32,
    // },

    // #[error("ConnectionError::ISE")]
    // ISE(#[source] AnyError),

    // #[error("ConnectionError::NoSuchVHost: {}", _0)]
    // NoSuchVHost(String),
}

impl ConnectionError {}
