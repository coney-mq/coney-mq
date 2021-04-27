use super::*;

impl<S> std::fmt::Debug for AmqpConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AmqpConnection<{}>", std::any::type_name::<S>()))
            .field("framing", &self.framing)
            .finish()
    }
}

impl<S> AmqpConnection<S> {
    pub fn new(framing: AmqpFraming<S>, backend: Arc<dyn Backend>) -> Self {
        Self { framing, backend }
    }
}

impl<S> AmqpConnection<S>
where
    S: IoStream,
{
    pub async fn run(mut self) -> Result<(), ConnectionError> {
        let handshake_result = handshake::run(&mut self.framing, self.backend.as_ref()).await;

        log::error!("not implemented: {:?}", handshake_result);

        unimplemented!()
    }
}
