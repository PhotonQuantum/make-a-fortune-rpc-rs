use mobc::{async_trait, Manager};
use rocket::tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Interest};
use rocket::tokio::net::TcpStream;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("connection closed unexpectedly")]
    RemoteClosed,
}

pub struct TcpConnection {
    tcp: TcpStream,
}

impl TcpConnection {
    pub async fn request(&mut self, data: &[u8]) -> Result<serde_json::Value, RequestError> {
        let (rx, mut tx) = self.tcp.split();
        let mut rx = BufReader::new(rx);

        tx.write_all(data).await?;

        let mut buf = String::new();
        rx.read_line(&mut buf).await?;

        Ok(serde_json::from_str(buf.as_str())?)
    }
}

pub struct TcpManager {
    address: String,
}

impl TcpManager {
    pub fn new(address: String) -> TcpManager {
        Self { address }
    }
}

#[async_trait]
impl Manager for TcpManager {
    type Connection = TcpConnection;
    type Error = RequestError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(&self.address).await?;

        Ok(TcpConnection { tcp })
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let status = conn
            .tcp
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await?;
        if status.is_read_closed() || status.is_write_closed() {
            Err(RequestError::RemoteClosed)
        } else {
            Ok(conn)
        }
    }
}
