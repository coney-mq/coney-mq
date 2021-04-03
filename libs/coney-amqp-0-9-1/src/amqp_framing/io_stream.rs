use ::tokio::io::AsyncRead;
use ::tokio::io::AsyncWrite;

pub trait IoStream: AsyncRead + AsyncWrite + Send + Sync + 'static {}

impl<T> IoStream for T where T: AsyncRead + AsyncWrite + Send + Sync + 'static {}
