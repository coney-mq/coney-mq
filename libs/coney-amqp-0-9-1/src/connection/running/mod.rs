use super::*;

use ::amq_protocol::frame::AMQPFrame;
use ::futures::channel::mpsc;
use ::futures::prelude::*;

use crate::amqp_framing::AmqpFrameProps;

pub async fn run<S>(
    framing: AmqpFraming<S>,
    conn_props: ConnProps,
    backend: Arc<dyn Backend>,
) -> Result<(), ConnectionError> {
    let (_send_queue_tx, mut send_queue_rx) =
        mpsc::channel::<AMQPFrame>(backend.amqp_config().send_queue_buf_size());

    unimplemented!()
}
