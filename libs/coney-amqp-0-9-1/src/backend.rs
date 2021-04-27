use std::sync::Arc;

use ::authc::Authc;
use ::common::AnyError;
use ::mq::vhost::VHost;

use crate::config::AmqpConfig;

#[async_trait::async_trait]
pub trait Backend: Send + Sync + 'static {
    fn amqp_config(&self) -> &dyn AmqpConfig;

    fn authc(&self) -> &dyn Authc;

    async fn vhost_select(&self, vhost_name: &str) -> Result<Option<Arc<dyn VHost>>, AnyError>;
}
