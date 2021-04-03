use super::*;

use ::amq_protocol::frame::AMQPFrame;

pub const CTL_CHANNEL_ID: u16 = 0;

pub fn expect_control_channel(channel_id: u16) -> Result<(), ConnectionError> {
    if channel_id != CTL_CHANNEL_ID {
        return Err(ConnectionError::NoChannel(channel_id));
    } else {
        Ok(())
    }
}

pub async fn receive_frame<S>(framing: &mut AmqpFraming<S>) -> Result<AMQPFrame, ConnectionError>
where
    S: IoStream,
{
    let frame = framing
        .recv()
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?
        .ok_or_else(|| ConnectionError::PeerGone)?;

    Ok(frame)
}
