use ::amq_protocol::frame::AMQPFrame;
use ::futures::stream::SplitSink;
use ::futures::stream::SplitStream;
use ::tokio_util::codec::Framed;

mod io_stream;
pub use io_stream::IoStream;

pub mod codec;
use codec::AmqpFrameCodec;
pub use codec::DecodeFailure;
pub use codec::EncodeFailure;

mod impl_amqp_framing;

pub struct AmqpFraming<S> {
    framed_write: SplitSink<Framed<S, AmqpFrameCodec>, AMQPFrame>,
    framed_read: SplitStream<Framed<S, AmqpFrameCodec>>,
}
