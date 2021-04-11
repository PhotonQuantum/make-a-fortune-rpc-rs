use rocket::tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use rocket::tokio::net::TcpStream;
use rocket::tokio::time::timeout as TokioTimeout;
use thiserror::Error;

use crate::config::Timeout;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("io timeout")]
    TimeoutError(#[from] rocket::tokio::time::error::Elapsed),
}

pub struct TcpConnection {
    tcp: TcpStream,
    timeout: Timeout,
}

impl TcpConnection {
    pub async fn connect(address: impl AsRef<str>, timeout: Timeout) -> Result<Self, RequestError> {
        let tcp = TokioTimeout(timeout.connect, TcpStream::connect(address.as_ref())).await??;

        Ok(TcpConnection { tcp, timeout })
    }

    pub async fn request(&mut self, data: &[u8]) -> Result<String, RequestError> {
        let (rx, mut tx) = self.tcp.split();
        let mut rx = BufReader::new(rx);

        TokioTimeout(self.timeout.write, tx.write_all(data)).await??;

        let mut buf = String::new();
        TokioTimeout(self.timeout.read, rx.read_line(&mut buf)).await??;

        Ok(buf)
    }
}
