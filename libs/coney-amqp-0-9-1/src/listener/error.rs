use ::common::AnyError;

#[derive(::thiserror::Error, Debug)]
pub enum AcceptorError {
    #[error("")]
    Accept(#[source] AnyError),

    #[error("")]
    SinkSendError,
}
