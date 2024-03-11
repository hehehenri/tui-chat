use std::net::SocketAddr;

use server::Server;
use transport::UdpTransport;
use uuid::Uuid;

mod message;
mod server;
mod transport;

type PeerId = Uuid;

#[derive(Debug)]
struct Peer {
    id: PeerId,
    addr: SocketAddr,
}

impl Peer {
    fn new(addr: SocketAddr) -> Self {
        Self {
            id: Uuid::new_v4(),
            addr,
        }
    }
}

fn main() {
    let transport = UdpTransport::new("0.0.0.0:6969").expect("failed to create transport layer");
    let server = Server::new(Box::new(transport));

    server.listen();
}
