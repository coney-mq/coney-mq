use std::sync::Arc;

use ::authc::Authc;
use ::common::AnyError;

use crate::amqp_exception::AmqpException;
use crate::amqp_framing::AmqpFraming;
use crate::amqp_framing::IoStream;
use crate::backend::Backend;

pub const CTL_CHANNEL_ID: u16 = 0;

mod props;
pub use props::ConnProps;

mod error;
pub use error::ConnectionError;
pub use error::ISE;

mod closing;
mod handshake;
mod impl_connection;
mod running;
mod util;

pub struct AmqpConnection<S> {
    backend: Arc<dyn Backend>,
    framing: AmqpFraming<S>,
}
