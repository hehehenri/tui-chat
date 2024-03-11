use std::{io, net::SocketAddr};

use async_trait::async_trait;
use tokio::net::UdpSocket;

pub const MAX_SIZE: usize = 1024;

pub struct TransportMessage {
    pub addr: SocketAddr,
    pub len: usize,
    pub content: String,
}

impl TransportMessage {
    fn new(addr: SocketAddr, len: usize, bytes: &[u8]) -> io::Result<Self> {
        let content = String::from_utf8(bytes.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        let message = TransportMessage { addr, len, content };
        Ok(message)
    }
}

#[async_trait]
pub trait Transport {
    async fn receive(&self) -> io::Result<TransportMessage>;
    async fn send_to(&self, message: Vec<u8>, addr: SocketAddr) -> io::Result<()>;
}

pub struct UdpTransport {
    socket: UdpSocket,
}

impl UdpTransport {
    pub async fn new(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;

        Ok(Self { socket })
    }
}

#[async_trait]
impl Transport for UdpTransport {
    async fn receive(&self) -> io::Result<TransportMessage> {
        let mut buf = [0; MAX_SIZE];
        let (len, addr) = self.socket.recv_from(&mut buf).await?;
        let bytes = buf[..len].to_vec();

        let message = TransportMessage::new(addr, len, &bytes)?;
        Ok(message)
    }

    async fn send_to(&self, message: Vec<u8>, addr: SocketAddr) -> io::Result<()> {
        self.socket.send_to(&message, addr).await?;

        Ok(())
    }
}
