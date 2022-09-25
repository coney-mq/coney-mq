use super::*;

use ::tokio_util::codec::Encoder;

use ::amq_protocol::frame::{gen_frame, WriteContext};

impl Encoder<AMQPFrame> for AmqpFrameCodec {
    type Error = EncodeFailure;

    fn encode(&mut self, item: AMQPFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        log::trace!("frame: {:?}", item);
        let size = match gen_frame(&item)(WriteContext::from(&mut self.enc_buf[..])) {
            Ok(write_context) => {
                let (_enc_buf_remainder, size) = write_context.into_inner();
                Ok(size)
            },
            Err(gen_error) => Err(EncodeFailure::GenError(gen_error)),
        }?;

        let () = dst.extend_from_slice(&self.enc_buf[..size as usize]);

        Ok(())
    }
}
