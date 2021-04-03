mod error;
pub use error::AcceptorError;

mod accept;
pub use accept::Accept;

mod impl_amqp_listener;
mod tcp;
mod uds;

#[derive(Debug)]
pub struct AmqpListener<A, S> {
    accept: A,
    sink: S,
}
