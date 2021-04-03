use ::bytes::*;

use ::amq_protocol::frame::AMQPFrame;

mod decode_failure;
pub use decode_failure::DecodeFailure;

mod encode_failure;
pub use encode_failure::EncodeFailure;

pub struct AmqpFrameCodec {
    enc_buf: Vec<u8>,
}
impl AmqpFrameCodec {
    pub fn new(enc_buf_size: u32) -> Self {
        Self {
            enc_buf: vec![0; enc_buf_size as usize],
        }
    }
}

mod impl_decoder;
mod impl_encoder;
