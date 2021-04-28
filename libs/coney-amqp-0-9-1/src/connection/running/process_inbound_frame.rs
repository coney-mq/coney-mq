use super::*;

pub(super) async fn process_inbound_frame<S>(
    framing: &mut AmqpFraming<S>,
    conn_channels: &mut ConnChannels,
    context: &mut ConnContext,
    inbound_frame: AMQPFrame,
) -> Result<LoopControl, ConnectionError>
where
    S: IoStream,
{
    let frame_props = AmqpFrameProps::from(&inbound_frame);

    let dispatch_result = if frame_props.channel_id == 0 {
        conn_channels
            .control_mut()
            .process_inbound_frame(context, inbound_frame)
            .await
    } else {
        dispatch_to_regular_channel(
            conn_channels,
            context,
            frame_props.channel_id,
            inbound_frame,
        )
        .await
    };
    let dispatch_result = dispatch_result.map_err(|e| e.with_props(frame_props));

    match dispatch_result {
        Ok(loop_control) => Ok(loop_control),
        Err(soft_exception) if soft_exception.is_soft() => {
            unimplemented!();
            Ok(LoopControl::Continue)
        }
        Err(hard_exception) => {
            let () = closing::run(framing, hard_exception).await?;
            Ok(LoopControl::Break)
        }
    }
}

async fn dispatch_to_regular_channel(
    conn_channels: &mut ConnChannels,
    context: &mut ConnContext,
    channel_id: u16,
    inbound_frame: AMQPFrame,
) -> Result<LoopControl, AmqpException> {
    conn_channels
        .regular_mut(channel_id)?
        .process_inbound_frame(inbound_frame)
        .await
}
