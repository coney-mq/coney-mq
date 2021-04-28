use super::*;

use ::amq_protocol::frame::AMQPFrame;
use ::amq_protocol::protocol::AMQPClass;
use ::futures::channel::mpsc;
use ::futures::prelude::*;

mod conn_context;
use conn_context::ConnContext;

mod conn_channels;
use conn_channels::ConnChannels;

mod control_channel;
use control_channel::ControlChannel;

mod regular_channel;
use regular_channel::RegularChannel;

mod conn_command;
use conn_command::ConnCommand;

mod process_outbound_frame;
use process_outbound_frame::process_outbound_frame;

mod process_inbound_frame;
use process_inbound_frame::process_inbound_frame;

mod process_conn_command;
use process_conn_command::process_conn_command;

use crate::amqp_exception::Condition;
use crate::amqp_framing::AmqpFrameProps;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopControl {
    Continue,
    Break,
}

pub async fn run<S>(
    framing: &mut AmqpFraming<S>,
    conn_props: ConnProps,
    backend: Arc<dyn Backend>,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    let (send_queue_tx, mut send_queue_rx) =
        mpsc::channel::<AMQPFrame>(backend.amqp_config().send_queue_buf_size());

    let (conn_command_tx, mut conn_command_rx) =
        mpsc::channel::<ConnCommand>(backend.amqp_config().conn_command_buf_size());

    let mut conn_channels = ConnChannels::new(
        ControlChannel::new(),
        (1..conn_props.tuning.max_channels)
            .map(RegularChannel::new)
            .collect(),
    );

    let mut context = ConnContext::new(conn_props, send_queue_tx, conn_command_tx);

    loop {
        let loop_control = ::futures::select! {
            inbound_frame = framing.recv().fuse() => {
                let inbound_frame = inbound_frame.map_err(Into::into).map_err(ConnectionError::IO)?;
                let inbound_frame = inbound_frame.ok_or(ConnectionError::PeerGone)?;

                let loop_control = process_inbound_frame(framing, &mut conn_channels, &mut context, inbound_frame).await?;
                loop_control
            },

            outbound_frame = send_queue_rx.next() => {
                let outbound_frame = outbound_frame.ok_or(ISE::SendQueueEndOfStream)?;
                let () = process_outbound_frame(framing, outbound_frame).await?;
                LoopControl::Continue
            },

            conn_command = conn_command_rx.next() => {
                let conn_command = conn_command.ok_or(ISE::ConnCommandEndOfStream)?;
                let () = process_conn_command(framing, &mut context, conn_command).await?;
                LoopControl::Continue
            }
        };

        match loop_control {
            LoopControl::Break => {
                let () = std::mem::drop(context);
                let () = std::mem::drop(conn_channels);

                while let Some(outbound_frame) = send_queue_rx.next().await {
                    let () = framing
                        .send(outbound_frame)
                        .await
                        .map_err(Into::into)
                        .map_err(ConnectionError::IO)?;
                }
                framing
                    .flush()
                    .await
                    .map_err(Into::into)
                    .map_err(ConnectionError::IO)?;

                break Ok(());
            }
            LoopControl::Continue => continue,
        }
    }
}
