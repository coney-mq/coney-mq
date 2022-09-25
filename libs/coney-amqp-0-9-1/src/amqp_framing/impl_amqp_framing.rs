use super::*;

use ::futures::prelude::*;

const ENC_BUF_SIZE: u32 = 1024;

impl<S> AmqpFraming<S>
where
    S: IoStream,
{
    pub fn new(io_stream: S) -> Self {
        let amqp_codec = AmqpFrameCodec::new(ENC_BUF_SIZE);
        let framed_stream = Framed::new(io_stream, amqp_codec);
        let (framed_write, framed_read) = framed_stream.split();
        Self { framed_write, framed_read }
    }

    pub async fn recv(&mut self) -> Result<Option<AMQPFrame>, DecodeFailure> {
        self.framed_read.next().await.transpose()
    }

    pub async fn send(&mut self, frame: AMQPFrame) -> Result<(), EncodeFailure> {
        self.framed_write.send(frame).await
    }

    pub async fn flush(&mut self) -> Result<(), EncodeFailure> {
        self.framed_write.flush().await
    }
}

impl<S> std::fmt::Debug for AmqpFraming<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AmqpFraming<{}>", std::any::type_name::<S>())).finish()
    }
}
