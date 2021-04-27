use ::common::AnyError;

mod condition;
pub use condition::Condition;

mod props;
pub use props::Props;

mod impl_amqp_exception;

#[derive(Debug, ::thiserror::Error)]
#[error("AmqpException[{} at {}]: {}", condition, props, message)]
pub struct AmqpException {
    condition: Condition,
    props: Props,
    message: String,

    #[source]
    source: Option<AnyError>,
}
