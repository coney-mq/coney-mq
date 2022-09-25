use ::tokio::io::{AsyncRead, AsyncWrite};

pub trait IoStream: AsyncRead + AsyncWrite + Send + Sync + 'static {}

impl<T> IoStream for T where T: AsyncRead + AsyncWrite + Send + Sync + 'static {}
