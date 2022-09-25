use super::*;

use ::futures::prelude::*;

impl<A, S> AmqpListener<A, S> {
    pub fn new(accept: A, sink: S) -> Self {
        Self { accept, sink }
    }
}

impl<A, S> AmqpListener<A, S>
where
    A: Accept,
    S: Sink<A::Conn>,
{
    pub async fn run(self) -> Result<(), AcceptorError> {
        let mut accept = self.accept;
        let sink = self.sink;

        ::futures::pin_mut!(sink);

        loop {
            let conn = accept.accept().await.map_err(Into::into).map_err(AcceptorError::Accept)?;
            let () = sink.send(conn).await.map_err(|_| AcceptorError::SinkSendError)?;
        }
    }
}
