use super::*;

use ::amq_protocol::frame::AMQPFrame;
use ::amq_protocol::frame::ProtocolVersion;

use crate::amqp_framing::AmqpFraming;

mod error;
pub use error::HandshakeError;

mod connection_open;
mod connection_start;
mod connection_tune;
mod receive_protocol_header;

pub use connection_tune::Tuning;

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    backend: &dyn Backend,
) -> Result<State, HandshakeError>
where
    S: IoStream,
{
    log::trace!("handshake...");
    let protocol_version = receive_protocol_header::run(framing).await?;
    log::trace!("protocol-version: {}", protocol_version);
    let identity = connection_start::run(framing, backend.authc()).await?;
    log::trace!("identity: {:?}", identity);
    let tuning = connection_tune::run(framing, backend.amqp_config().connection_limits()).await?;
    log::trace!("tuning: {:?}", tuning);
    let (vhost_name, vhost_api) = connection_open::run(framing, backend).await?;

    let state = State {
        protocol_version,
        identity,
        tuning,
        vhost_name,
        vhost_api,
    };

    Ok(state)
}

pub const CTL_CHANNEL_ID: u16 = 0;

fn expect_control_channel(channel_id: u16) -> Result<(), HandshakeError> {
    if channel_id != CTL_CHANNEL_ID {
        return Err(HandshakeError::ExpectedControlChannel { channel_id });
    } else {
        Ok(())
    }
}
