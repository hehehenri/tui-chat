use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

pub const MAX_SIZE: usize = 1024;

pub trait Transport {
    fn receive(&self) -> io::Result<Vec<u8>>;
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
    fn receive(&self) -> io::Result<Vec<u8>> {
        let mut buf = [0; MAX_SIZE];
        let (len, _) = self.socket.recv_from(&mut buf)?;
        let message = buf[..len].to_vec();

        Ok(message)
    }

    fn send_to(&self, message: Vec<u8>, addr: SocketAddr) -> io::Result<()> {
        self.socket.send_to(&message, addr)?;

        Ok(())
    }
}
