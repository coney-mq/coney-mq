pub trait ConnectionLimits: Send + Sync + 'static {
    fn max_channels(&self) -> u16;
    fn max_frame_size(&self) -> u32;
    fn max_heartbeat(&self) -> u16;
}
