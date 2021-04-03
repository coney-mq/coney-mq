use super::*;

impl<S> std::fmt::Debug for AmqpConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AmqpConnection<{}>", std::any::type_name::<S>()))
            .field("framing", &self.framing)
            .finish()
    }
}

impl<S> AmqpConnection<S> {
    pub fn new(
        framing: AmqpFraming<S>,
        authc: Arc<dyn Authc>,
        config: Arc<dyn AmqpConfig>,
    ) -> Self {
        Self {
            framing,
            authc,
            config,
        }
    }
}

impl<S> AmqpConnection<S>
where
    S: IoStream,
{
    pub async fn run(mut self) -> Result<(), ConnectionError> {
        let _state =
            handshake::run(&mut self.framing, self.authc.as_ref(), self.config.as_ref()).await?;
        unimplemented!()
    }
}
