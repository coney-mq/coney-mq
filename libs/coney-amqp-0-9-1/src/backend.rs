use ::authc::Authc;

use crate::config::AmqpConfig;

#[async_trait::async_trait]
pub trait Backend: Send + Sync + 'static {
    fn amqp_config(&self) -> &dyn AmqpConfig;

    fn authc(&self) -> &dyn Authc;
}
