use super::*;

pub(super) async fn process_outbound_frame<S>(
    amqp_framing: &mut AmqpFraming<S>,
    outbound_frame: AMQPFrame,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    log::trace!("outbound-frame: {:?}", outbound_frame);
    let () = amqp_framing
        .send(outbound_frame)
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?;

    Ok(())
}
