use ::amq_protocol::frame::AMQPFrame;
use ::futures::stream::{SplitSink, SplitStream};
use ::tokio_util::codec::Framed;

mod amqp_frame_props;
pub use amqp_frame_props::AmqpFrameProps;

mod io_stream;
pub use io_stream::IoStream;

pub mod codec;
use codec::AmqpFrameCodec;
pub use codec::{DecodeFailure, EncodeFailure};

mod impl_amqp_framing;

pub struct AmqpFraming<S> {
    framed_write: SplitSink<Framed<S, AmqpFrameCodec>, AMQPFrame>,
    framed_read: SplitStream<Framed<S, AmqpFrameCodec>>,
}
