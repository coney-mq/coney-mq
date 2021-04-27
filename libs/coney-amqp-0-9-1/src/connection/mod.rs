use std::sync::Arc;

use ::authc::Authc;
use ::common::AnyError;

use crate::amqp_exception::AmqpException;
use crate::amqp_framing::AmqpFraming;
use crate::amqp_framing::IoStream;
use crate::backend::Backend;

mod state;
use state::State;

mod error;
pub use error::ConnectionError;
pub use error::ISE;

mod handshake;
mod impl_connection;
mod util;

pub struct AmqpConnection<S> {
    backend: Arc<dyn Backend>,
    framing: AmqpFraming<S>,
}
