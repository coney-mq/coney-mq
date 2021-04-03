use std::io;

use ::amq_protocol_types::parsing::ParserErrors;

#[derive(Debug, ::thiserror::Error)]
pub enum DecodeFailure {
    #[error("DecodeFailure::Unimplemented")]
    Unimplemented,

    #[error("DecodeFailure::IO")]
    IO(#[source] io::Error),

    #[error("DecodeFailure::ParseError")]
    ParseError(#[source] ParserErrors),
}

impl From<io::Error> for DecodeFailure {
    fn from(v: io::Error) -> Self {
        Self::IO(v)
    }
}
