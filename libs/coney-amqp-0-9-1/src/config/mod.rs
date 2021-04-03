mod connection_limits;
pub use connection_limits::ConnectionLimits;

pub trait AmqpConfig: Send + Sync + 'static {
    fn connection_limits(&self) -> &dyn ConnectionLimits;
}
