use std::io;

use ::amq_protocol::frame::GenError;

#[derive(Debug, ::thiserror::Error)]
pub enum EncodeFailure {
    #[error("EncodeFailure::Unimplemented")]
    Unimplemented,

    #[error("EncodeFailure::IO")]
    IO(#[source] io::Error),

    #[error("EncodeFailure::GenError")]
    GenError(#[source] GenError),
}

impl From<io::Error> for EncodeFailure {
    fn from(v: io::Error) -> Self {
        Self::IO(v)
    }
}
