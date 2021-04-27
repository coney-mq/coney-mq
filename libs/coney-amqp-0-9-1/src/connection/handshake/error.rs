use super::*;

use crate::amqp_exception::Condition;
use crate::amqp_exception::Props;

#[derive(Debug, ::thiserror::Error)]
pub enum HandshakeError {
    #[error("HandshakeError::ExpectedControlChannel [ch-id: {}]", props.channel_id)]
    ExpectedControlChannel { props: Props },

    #[error("HandshakeError::RecvError")]
    RecvError(#[source] util::RecvError),

    #[error("HandshakeERror::SendError")]
    SendError(#[source] AnyError),

    #[error("HandshakeError::UnexpectedFrame [expected: {}]", expected)]
    UnexpectedFrame {
        expected: &'static str,
        props: Props,
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
    ISE {
        props: Props,
        #[source]
        source: AnyError,
    },
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

impl HandshakeError {
    pub fn into_amqp_exception(self) -> Result<AmqpException, ConnectionError> {
        let amqp_exception = match self {
            Self::ExpectedControlChannel { props, .. } => {
                AmqpException::new("expected control channel")
                    .with_condition(Condition::ChannelError)
                    .with_props(props)
                    .with_source(self)
            }
            Self::UnexpectedFrame { props, .. } => AmqpException::new("invalid command")
                .with_condition(Condition::CommandInvalid)
                .with_props(props)
                .with_source(self),
            Self::AuthcTooManyChallenges { .. } => {
                AmqpException::new("too many authentication challenge attempts")
                    .with_condition(Condition::NotAllowed)
                    .with_props(make_props(CID_CONN, MID_CONN_SECURE_OK))
                    .with_source(self)
            }
            Self::AuthcMechError { .. } => AmqpException::new("authentication failure")
                .with_condition(Condition::NotAllowed)
                .with_props(make_props(CID_CONN, MID_CONN_SECURE_OK))
                .with_source(self),
            Self::TuneNegotiationError { .. } => AmqpException::new("Tune negotiation error")
                .with_condition(Condition::ResourceError)
                .with_props(make_props(CID_CONN, MID_CONN_TUNE))
                .with_source(self),

            Self::NoSuchVHost { .. } => AmqpException::new("Virtual host does not exist")
                .with_condition(Condition::InvalidPath)
                .with_props(make_props(CID_CONN, MID_CONN_OPEN))
                .with_source(self),

            Self::ISE { props, .. } => AmqpException::new("Internal Error")
                .with_condition(Condition::InternalError)
                .with_source(self),

            _ => return Err(ConnectionError::HandshakeError(self)),
        };
        Ok(amqp_exception)
    }
}

const CID_CONN: u16 = 10;
const MID_CONN_SECURE_OK: u16 = 21;
const MID_CONN_TUNE: u16 = 30;
const MID_CONN_OPEN: u16 = 40;

fn make_props(class_id: u16, method_id: u16) -> Props {
    Props {
        channel_id: CTL_CHANNEL_ID,
        class_id,
        method_id,
    }
}
