use std::sync::Arc;

use ::authc::Authc;

use crate::amqp_framing::AmqpFraming;
use crate::amqp_framing::IoStream;
use crate::backend::Backend;

mod error;
pub use error::ConnectionError;

mod handshake;
mod impl_connection;
mod util;

pub struct AmqpConnection<S> {
    backend: Arc<dyn Backend>,
    framing: AmqpFraming<S>,
}
