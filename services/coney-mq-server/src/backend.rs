use std::collections::HashMap;
use std::sync::Arc;

use ::amqp_0_9_1::backend::Backend;
use ::amqp_0_9_1::config::AmqpConfig;
use ::amqp_0_9_1::config::ConnectionLimits;
use ::authc::Authc;
use ::authc::AuthcWithMechs;
use ::common::AnyError;
use ::mq::vhost::VHost;

pub struct BE {
    pub authc: AuthcWithMechs,
    pub vhosts: HashMap<String, Arc<dyn VHost>>,
}
impl AmqpConfig for BE {
    fn connection_limits(&self) -> &dyn ConnectionLimits {
        self
    }
    fn send_queue_buf_size(&self) -> usize {
        64
    }
    fn conn_command_buf_size(&self) -> usize {
        64
    }
}
impl ConnectionLimits for BE {
    fn max_channels(&self) -> u16 {
        512
    }
    fn max_frame_size(&self) -> u32 {
        10240
    }
    fn max_heartbeat(&self) -> u16 {
        300
    }
}
#[async_trait::async_trait]
impl Backend for BE {
    fn amqp_config(&self) -> &dyn AmqpConfig {
        self
    }
    fn authc(&self) -> &dyn Authc {
        &self.authc
    }

    async fn vhost_select(&self, vhost_name: &str) -> Result<Option<Arc<dyn VHost>>, AnyError> {
        Ok(self.vhosts.get(vhost_name).cloned())
    }
}

#[derive(Debug)]
pub struct VH {}

mod exchange_declare {
    use super::*;
    use ::mq::vhost::exchange_declare::*;

    #[async_trait::async_trait]
    impl ExchangeDeclare for VH {
        async fn exchange_declare(
            &self,
            _rq: ExchangeDeclareRq,
        ) -> Result<ExchangeDeclareOk, ExchangeDeclareErr> {
            unimplemented!()
        }
    }
}

mod exchange_delete {
    use super::*;
    use ::mq::vhost::exchange_delete::*;

    #[async_trait::async_trait]
    impl ExchangeDelete for VH {
        async fn exchange_delete(
            &self,
            _rq: ExchangeDeleteRq,
        ) -> Result<ExchangeDeleteOk, ExchangeDeleteErr> {
            unimplemented!()
        }
    }
}

mod exchange_inspect {
    use super::*;
    use ::mq::vhost::exchange_inspect::*;

    #[async_trait::async_trait]
    impl ExchangeInspect for VH {
        async fn exchange_inspect(
            &self,
            _rq: ExchangeInspectRq,
        ) -> Result<ExchangeInspectOk, ExchangeInspectErr> {
            unimplemented!()
        }
    }
}
