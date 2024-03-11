use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

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

pub trait Transport {
    fn receive(&self) -> io::Result<TransportMessage>;
    fn send_to(&self, message: Vec<u8>, addr: SocketAddr) -> io::Result<()>;
}

pub struct UdpTransport {
    socket: UdpSocket,
}

impl UdpTransport {
    pub fn new(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self { socket })
    }
}

impl Transport for UdpTransport {
    fn receive(&self) -> io::Result<TransportMessage> {
        let mut buf = [0; MAX_SIZE];
        let (len, addr) = self.socket.recv_from(&mut buf)?;
        let bytes = buf[..len].to_vec();

        let message = TransportMessage::new(addr, len, &bytes)?;
        Ok(message)
    }

    fn send_to(&self, message: Vec<u8>, addr: SocketAddr) -> io::Result<()> {
        self.socket.send_to(&message, addr)?;

        Ok(())
    }
}
