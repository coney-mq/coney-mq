#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ::thiserror::Error)]
pub enum Condition {
    #[error("CONNECTION_FORCED")]
    ConnectionForced,

    #[error("INVALID_PATH")]
    InvalidPath,

    #[error("FRAME_ERROR")]
    FrameError,

    // SyntaxError,
    #[error("COMMAND_INVALID")]
    CommandInvalid,

    #[error("CHANNEL_ERROR")]
    ChannelError,

    #[error("UNEXPECTED_FRAME")]
    UnexpectedFrame,

    #[error("RESOURCE_ERROR")]
    ResourceError,

    #[error("NOT_ALLOWED")]
    NotAllowed,

    #[error("NOT_IMPLEMENTED")]
    NotImplemented,

    #[error("INTERNAL_ERROR")]
    InternalError,

    #[error("CONTENT_TOO_LARGE")]
    ContentTooLarge,

    #[error("NO_ROUTE")]
    NoRoute,

    #[error("NO_CONSUMERS")]
    NoConsumers,

    #[error("ACCESS_REFUSED")]
    AccessRefused,

    #[error("NOT_FOUND")]
    NotFound,

    #[error("RESOURCE_LOCKED")]
    ResourceLocked,

    #[error("PRECONDIION_FAILED")]
    PreconditionFailed,
}

impl Default for Condition {
    fn default() -> Self {
        Self::InternalError
    }
}

impl Condition {
    pub fn id(&self) -> u16 {
        match self {
            Self::ConnectionForced => 320,
            Self::InvalidPath => 402,
            Self::FrameError => 501,
            Self::CommandInvalid => 503,
            Self::ChannelError => 504,
            Self::UnexpectedFrame => 505,
            Self::ResourceError => 506,
            Self::NotAllowed => 530,
            Self::NotImplemented => 540,
            Self::InternalError => 541,

            Self::ContentTooLarge => 311,
            Self::NoRoute => 312,
            Self::NoConsumers => 313,
            Self::AccessRefused => 403,
            Self::NotFound => 404,
            Self::ResourceLocked => 405,
            Self::PreconditionFailed => 406,
        }
    }

    pub fn is_hard(&self) -> bool {
        match self {
            Self::ConnectionForced => true,
            Self::InvalidPath => true,
            Self::FrameError => true,
            Self::CommandInvalid => true,
            Self::ChannelError => true,
            Self::UnexpectedFrame => true,
            Self::ResourceError => true,
            Self::NotAllowed => true,
            Self::NotImplemented => true,
            Self::InternalError => true,

            _ => false,
        }
    }
    pub fn is_soft(&self) -> bool {
        !self.is_hard()
    }
}
