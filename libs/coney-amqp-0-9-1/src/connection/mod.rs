use std::sync::Arc;

use ::authc::Authc;

use crate::amqp_framing::AmqpFraming;
use crate::amqp_framing::IoStream;
use crate::config::AmqpConfig;

mod error;
pub use error::ConnectionError;

mod handshake;
mod impl_connection;
mod util;

pub struct AmqpConnection<S> {
    config: Arc<dyn AmqpConfig>,
    authc: Arc<dyn Authc>,
    framing: AmqpFraming<S>,
}
