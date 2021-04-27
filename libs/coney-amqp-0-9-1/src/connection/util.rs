use super::*;

use ::amq_protocol::frame::AMQPFrame;

#[derive(Debug, ::thiserror::Error)]
pub enum RecvError {
    #[error("RecvError::PeerGone")]
    PeerGone,

    #[error("RecvError::IO")]
    IO(#[source] AnyError),
}

pub async fn receive_frame<S>(framing: &mut AmqpFraming<S>) -> Result<AMQPFrame, RecvError>
where
    S: IoStream,
{
    let frame = framing
        .recv()
        .await
        .map_err(Into::into)
        .map_err(RecvError::IO)?
        .ok_or(RecvError::PeerGone)?;

    Ok(frame)
}
