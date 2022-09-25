use super::*;

#[derive(Debug)]
pub(super) struct ConnChannels {
    control_channel: ControlChannel,
    regular_channels: Vec<RegularChannel>,
}
impl ConnChannels {
    pub fn new(control_channel: ControlChannel, regular_channels: Vec<RegularChannel>) -> Self {
        Self { control_channel, regular_channels }
    }

    pub fn control_mut(&mut self) -> &mut ControlChannel {
        &mut self.control_channel
    }
    pub fn regular_mut(&mut self, chan_id: u16) -> Result<&mut RegularChannel, AmqpException> {
        if chan_id == 0 {
            Err(AmqpException::new(
                "Attempted to fetch control-channel from the regular-channels collection",
            )
            .with_condition(Condition::InternalError))?
        }
        self.regular_channels.get_mut(chan_id as usize - 1).ok_or_else(|| {
            AmqpException::new("Attempted to access the channel beyond the quantity of channels")
                .with_condition(Condition::ChannelError)
        })
    }
}
