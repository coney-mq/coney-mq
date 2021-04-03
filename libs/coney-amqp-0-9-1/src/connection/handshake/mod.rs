use super::*;

use ::amq_protocol::frame::AMQPFrame;

use crate::amqp_framing::AmqpFraming;

mod connection_open;
mod connection_start;
mod connection_tune;
mod receive_protocol_header;

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    authc: &dyn Authc,
    config: &dyn AmqpConfig,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    log::trace!("handshake...");
    let protocol_version = receive_protocol_header::run(framing).await?;
    log::trace!("protocol-version: {}", protocol_version);
    let identity = connection_start::run(framing, authc).await?;
    log::trace!("identity: {:?}", identity);
    let tuning = connection_tune::run(framing, config.connection_limits()).await?;
    log::trace!("tuning: {:?}", tuning);
    let _ = connection_open::run(framing).await?;

    log::error!("handshake unimplemented!");

    unimplemented!()
}
