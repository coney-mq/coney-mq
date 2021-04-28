use super::*;

use ::amq_protocol::frame::AMQPFrame;

use ::common::ErrorReport;

use handshake::HandshakeError;

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
        let conn_props = match handshake::run(&mut self.framing, self.backend.as_ref()).await {
            Err(handshake_error) => {
                return process_handshake_error(&mut self.framing, handshake_error).await
            }
            Ok(state) => state,
        };

        log::trace!("connection initialized: {:#?}", conn_props);

        running::run(&mut self.framing, conn_props, self.backend).await
    }
}

async fn process_handshake_error<S>(
    framing: &mut AmqpFraming<S>,
    handshake_error: HandshakeError,
) -> Result<(), ConnectionError>
where
    S: IoStream,
{
    let amqp_exception = handshake_error.into_amqp_exception()?;
    let () = closing::run(framing, amqp_exception).await?;
    Ok(())
}
