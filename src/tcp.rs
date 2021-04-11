use rocket::tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use rocket::tokio::net::TcpStream;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

pub struct TcpConnection {
    tcp: TcpStream,
}

impl TcpConnection {
    pub async fn connect(address: impl AsRef<str>) -> Result<Self, RequestError> {
        let tcp = TcpStream::connect(address.as_ref()).await?;

        Ok(TcpConnection { tcp })
    }

    pub async fn request(&mut self, data: &[u8]) -> Result<String, RequestError> {
        let (rx, mut tx) = self.tcp.split();
        let mut rx = BufReader::new(rx);

        tx.write_all(data).await?;

        let mut buf = String::new();
        rx.read_line(&mut buf).await?;

        Ok(buf)
    }
}
