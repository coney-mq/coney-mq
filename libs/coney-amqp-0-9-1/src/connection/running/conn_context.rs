use super::*;

#[derive(Debug)]
pub(super) struct ConnContext {
    conn_props: ConnProps,
    send_queue_tx: mpsc::Sender<AMQPFrame>,
    conn_command_tx: mpsc::Sender<ConnCommand>,
}

impl ConnContext {
    pub fn new(
        conn_props: ConnProps,
        send_queue_tx: mpsc::Sender<AMQPFrame>,
        conn_command_tx: mpsc::Sender<ConnCommand>,
    ) -> Self {
        Self { conn_props, send_queue_tx, conn_command_tx }
    }

    pub async fn send_frame(&mut self, frame: AMQPFrame) -> Result<(), AmqpException> {
        self.send_queue_tx.send(frame).await.map_err(|err| {
            AmqpException::new("Failed to enqueue outbound frame")
                .with_condition(Condition::InternalError)
                .with_source(err)
        })
    }
}
