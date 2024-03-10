use std::{
    io::{self, IoSlice},
    net::{Ipv4Addr, UdpSocket},
};

use server::Server;
use transport::UdpTransport;

mod server;
mod transport;

type PeerId = [u8; 20];

struct PeerAddr {
    ip: Ipv4Addr,
    port: u16,
}

struct Peer {
    id: PeerId,
    addr: PeerAddr,
}

fn main() {
    let transport = UdpTransport::new("0.0.0.0:6969").expect("failed to create transport layer");
    let server = Server::new(Box::new(transport));

    server.listen();
}
