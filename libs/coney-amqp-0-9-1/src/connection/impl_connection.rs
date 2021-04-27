use super::*;

use ::common::ErrorReport;

use handshake::HandshakeError;

impl<S> std::fmt::Debug for AmqpConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AmqpConnection<{}>", std::any::type_name::<S>()))
            .field("framing", &self.framing)
            .finish()
    }
}

impl<S> AmqpConnection<S> {
    pub fn new(framing: AmqpFraming<S>, backend: Arc<dyn Backend>) -> Self {
        Self { framing, backend }
    }
}

impl<S> AmqpConnection<S>
where
    S: IoStream,
{
    pub async fn run(mut self) -> Result<(), ConnectionError> {
        let state = match handshake::run(&mut self.framing, self.backend.as_ref()).await {
            Err(handshake_error) => {
                return process_handshake_error(&mut self.framing, handshake_error).await
            }
            Ok(state) => state,
        };

        log::error!("not implemented: {:#?}", state);

        unimplemented!()
    }
}

async fn process_handshake_error<S>(
    framing: &mut AmqpFraming<S>,
    handshake_error: HandshakeError,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    use ::amq_protocol::frame::AMQPFrame;
    use ::amq_protocol::protocol::connection::AMQPMethod as AmqpMethodConn;
    use ::amq_protocol::protocol::AMQPClass;

    let amqp_exception = handshake_error.into_amqp_exception()?;
    let reply_code = amqp_exception.condition().id();
    let reply_text = amqp_exception.error_report();
    let class_id = amqp_exception.props().class_id;
    let method_id = amqp_exception.props().method_id;
    let channel_id = handshake::CTL_CHANNEL_ID;

    let pdu = ::amq_protocol::protocol::connection::Close {
        class_id,
        method_id,
        reply_code,
        reply_text: reply_text.into(),
    };
    let close_rq = AmqpMethodConn::Close(pdu);
    let close_rq = AMQPClass::Connection(close_rq);
    let close_rq = AMQPFrame::Method(channel_id, close_rq);

    let () = framing
        .send(close_rq)
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?;

    if let Some(close_rs) = framing
        .recv()
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?
    {
        match close_rs {
            AMQPFrame::Method(
                handshake::CTL_CHANNEL_ID,
                AMQPClass::Connection(AmqpMethodConn::CloseOk(_close_ok)),
            ) => (),

            unexpected => log::warn!(
                "Expected nothing but Connection/Close-Ok. Received: {:?}. Giving up.",
                unexpected
            ),
        }
    }

    Ok(())
}
