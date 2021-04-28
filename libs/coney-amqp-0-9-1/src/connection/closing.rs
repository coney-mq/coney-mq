use super::*;

use ::amq_protocol::frame::AMQPFrame;
use ::amq_protocol::protocol::connection::AMQPMethod as AmqpMethodConn;
use ::amq_protocol::protocol::connection::Close as ConnClose;
use ::amq_protocol::protocol::connection::CloseOk as ConnCloseOk;
use ::amq_protocol::protocol::AMQPClass;

use ::common::ErrorReport;

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    amqp_exception: AmqpException,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    let () = send_conn_close(framing, amqp_exception).await?;

    while let Some(inbound_frame) = framing
        .recv()
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?
    {
        match inbound_frame {
            AMQPFrame::Method(
                CTL_CHANNEL_ID,
                AMQPClass::Connection(AmqpMethodConn::CloseOk(_close_ok)),
            ) => {
                log::trace!("Received Conn/Close-Ok. Shutting down.");
                return Ok(());
            }

            AMQPFrame::Method(
                CTL_CHANNEL_ID,
                AMQPClass::Connection(AmqpMethodConn::Close(close)),
            ) => {
                log::trace!(
                    "The connection is closing. The peer also requested Conn/Close: {:?}",
                    close
                );

                let close_ok = ConnCloseOk {};
                let close_ok = AmqpMethodConn::CloseOk(close_ok);
                let close_ok = AMQPClass::Connection(close_ok);
                let close_ok = AMQPFrame::Method(CTL_CHANNEL_ID, close_ok);

                framing
                    .send(close_ok)
                    .await
                    .map_err(Into::into)
                    .map_err(ConnectionError::IO)?;

                let () = maybe_receive_close_ok(framing).await?;
                return Ok(());
            }

            to_ignore => log::trace!("The connection is closing. Ignoring {:?}", to_ignore),
        }
    }
    log::trace!("The connection is closed from the other side.");
    Ok(())
}

async fn maybe_receive_close_ok<S>(framing: &mut AmqpFraming<S>) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    while let Some(inbound_frame) = framing
        .recv()
        .await
        .map_err(Into::into)
        .map_err(ConnectionError::IO)?
    {
        match inbound_frame {
            AMQPFrame::Method(
                CTL_CHANNEL_ID,
                AMQPClass::Connection(AmqpMethodConn::CloseOk(_close_ok)),
            ) => {
                log::trace!("Received Conn/Close-Ok. Shutting down.");
                return Ok(());
            }

            to_ignore => log::trace!("The connection is closing. Ignoring {:?}", to_ignore),
        }
    }
    Ok(())
}

async fn send_conn_close<S>(
    framing: &mut AmqpFraming<S>,
    amqp_exception: AmqpException,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    let reply_code = amqp_exception.condition().id();
    let reply_text = amqp_exception.error_report();
    let class_id = amqp_exception.props().class_id;
    let method_id = amqp_exception.props().method_id;
    let channel_id = CTL_CHANNEL_ID;

    let pdu = ConnClose {
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
