use ::common::AnyError;

use crate::amqp_framing::AmqpFrameProps;

mod condition;
pub use condition::Condition;

mod impl_amqp_exception;

#[derive(Debug, ::thiserror::Error)]
#[error("AmqpException[{} at {}]: {}", condition, caused_by, message)]
pub struct AmqpException {
    condition: Condition,
    caused_by: AmqpFrameProps,
    message: String,

    #[source]
    source: Option<AnyError>,
}
