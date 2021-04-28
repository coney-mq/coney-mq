use super::*;

use ::amq_protocol::frame::AMQPFrame;

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
        let conn_props = match handshake::run(&mut self.framing, self.backend.as_ref()).await {
            Err(handshake_error) => {
                return process_handshake_error(&mut self.framing, handshake_error).await
            }
            Ok(state) => state,
        };

        log::trace!("connection initialized: {:#?}", conn_props);

        // loop {
        //     ::futures::select! {
        //         outbound_frame = send_queue_rx.next().fuse() => {
        //             let outbound_frame = outbound_frame.ok_or(ConnectionError::ISE(ISE::SendQueueEndOfStream))?;
        //             let () = self.framing.send(outbound_frame).await.map_err(Into::into).map_err(ConnectionError::IO)?;
        //         },
        //         inbound_frame = self.framing.recv().fuse() => {
        //             let inbound_frame = inbound_frame.map_err(Into::into).map_err(ConnectionError::IO)?.ok_or(ConnectionError::PeerGone)?;
        //             let frame_props = AmqpFrameProps::from(&inbound_frame);

        //             let channel_id = frame_props.channel_id;
        //             let channel = channels
        //                 .channel_mut(channel_id)
        //                 .ok_or_else(||
        //                     AmqpException::new("channel-id is out of bounds")
        //                         .with_props(frame_props)
        //                         .with_condition(crate::amqp_exception::Condition::ChannelError));
        //             let channel = match channel {
        //                 Ok(channel) => channel,
        //                 Err(amqp_exception) => {
        //                     if !process_amqp_exception(&mut self.framing, amqp_exception).await? {
        //                         continue
        //                     } else {
        //                         return Ok(())
        //                     }
        //                 }
        //             };

        //             let dispatched = channel.dispatch_frame(inbound_frame, &conn_props).await;
        //             let () = match dispatched {
        //                 Ok(dispatched) => dispatched,
        //                 Err(amqp_exception) => {
        //                     if !process_amqp_exception(&mut self.framing, amqp_exception).await? {
        //                         continue
        //                     } else {
        //                         return Ok(())
        //                     }
        //                 }
        //             };
        //         }
        //     }
        // }

        running::run(self.framing, conn_props, self.backend).await
    }
}

async fn process_amqp_exception<S>(
    framing: &mut AmqpFraming<S>,
    amqp_exception: AmqpException,
) -> Result<bool, ConnectionError>
where
    S: IoStream,
{
    let is_hard = amqp_exception.is_hard();

    use ::amq_protocol::protocol::channel::AMQPMethod as AmqpMethodChan;
    use ::amq_protocol::protocol::connection::AMQPMethod as AmqpMethodConn;
    use ::amq_protocol::protocol::AMQPClass;

    let reply_code = amqp_exception.condition().id();
    let reply_text = amqp_exception.error_report();
    let class_id = amqp_exception.props().class_id;
    let method_id = amqp_exception.props().method_id;
    let channel_id = amqp_exception.props().channel_id;

    let frame = if is_hard {
        let close_rq = ::amq_protocol::protocol::connection::Close {
            class_id,
            method_id,
            reply_code,
            reply_text: reply_text.into(),
        };
        let close_rq = AmqpMethodConn::Close(close_rq);
        let close_rq = AMQPClass::Connection(close_rq);
        AMQPFrame::Method(CTL_CHANNEL_ID, close_rq)
    } else {
        let close_rq = ::amq_protocol::protocol::channel::Close {
            class_id,
            method_id,
            reply_code,
            reply_text: reply_text.clone().into(),
        };
        let close_rq = AmqpMethodChan::Close(close_rq);
        let close_rq = AMQPClass::Channel(close_rq);
        AMQPFrame::Method(channel_id, close_rq)
    };

    let () = framing
        .send(frame)
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?;

    Ok(is_hard)
}

async fn process_hard_amqp_exception<S>(
    framing: &mut AmqpFraming<S>,
    amqp_exception: AmqpException,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    use ::amq_protocol::protocol::connection::AMQPMethod as AmqpMethodConn;
    use ::amq_protocol::protocol::AMQPClass;

    let reply_code = amqp_exception.condition().id();
    let reply_text = amqp_exception.error_report();
    let class_id = amqp_exception.props().class_id;
    let method_id = amqp_exception.props().method_id;
    let channel_id = CTL_CHANNEL_ID;

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
                CTL_CHANNEL_ID,
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

async fn process_handshake_error<S>(
    framing: &mut AmqpFraming<S>,
    handshake_error: HandshakeError,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    let amqp_exception = handshake_error.into_amqp_exception()?;
    let () = process_hard_amqp_exception(framing, amqp_exception).await?;

    Ok(())
}
