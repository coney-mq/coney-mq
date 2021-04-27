use super::*;

use ::amq_protocol::protocol::connection::AMQPMethod;
use ::amq_protocol::protocol::connection::Tune;
use ::amq_protocol::protocol::AMQPClass;
// use ::amq_protocol::protocol::connection::TuneOk;

use crate::config::ConnectionLimits;

#[derive(Debug)]
pub struct Tuning {
    pub max_frame_size: u32,
    pub max_channels: u16,
    pub max_heartbeat: u16,
}

/*
    On the wire:
        * Send Method:Connection/Tune
        * Receive Method:Connection/TuneOk

    Description:
        The server offers the connection limits:
            - the maximal quantity of channels;
            - the maximal frame size in bytes;
            - the maximal heartbeat interval.

        The client chooses the connection limits (may reduce the limits, must not increase):
            - the maximal quantity of channels;
            - the maximal frame size in bytes;
            - the maximal heartbeat interval.
*/

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    conn_limits: &dyn ConnectionLimits,
) -> Result<Tuning, HandshakeError>
where
    S: IoStream,
{
    let max_channels = conn_limits.max_channels();
    let max_frame_size = conn_limits.max_frame_size();
    let max_heartbeat = conn_limits.max_heartbeat();

    let tune = Tune {
        channel_max: max_channels,
        frame_max: max_frame_size,
        heartbeat: max_heartbeat,
    };
    let method = AMQPMethod::Tune(tune);
    let class = AMQPClass::Connection(method);
    let frame = AMQPFrame::Method(CTL_CHANNEL_ID, class);

    let () = framing
        .send(frame)
        .await
        .map_err(Into::into)
        .map_err(HandshakeError::SendError)?;

    let frame = util::receive_frame(framing).await?;

    match frame {
        AMQPFrame::Method(channel_id, AMQPClass::Connection(AMQPMethod::TuneOk(tune_ok))) => {
            let () = expect_control_channel(
                channel_id,
                tune_ok.get_amqp_class_id(),
                tune_ok.get_amqp_method_id(),
            )?;

            Ok(Tuning {
                max_channels: expect_within_the_limit(
                    "channel_max",
                    max_channels,
                    tune_ok.channel_max,
                )?,
                max_frame_size: expect_within_the_limit(
                    "frame_max",
                    max_frame_size,
                    tune_ok.frame_max,
                )?,
                max_heartbeat: expect_within_the_limit(
                    "heartbeat",
                    max_heartbeat,
                    tune_ok.heartbeat,
                )?,
            })
        }
        unexpected => Err(HandshakeError::UnexpectedFrame {
            expected: "Method.Connection/Tune-Ok",
            props: From::from(&unexpected),
        })?,
    }
}

fn expect_within_the_limit<V>(
    field: &'static str,
    max: V,
    requested: V,
) -> Result<V, HandshakeError>
where
    V: ToU32,
{
    if requested > max {
        Err(HandshakeError::TuneNegotiationError {
            field,
            max: max.to_u32(),
            requested: requested.to_u32(),
        })
    } else {
        Ok(requested)
    }
}
trait ToU32: Copy + Ord {
    fn to_u32(self) -> u32;
}
impl ToU32 for u32 {
    fn to_u32(self) -> u32 {
        self
    }
}
impl ToU32 for u16 {
    fn to_u32(self) -> u32 {
        self as u32
    }
}
