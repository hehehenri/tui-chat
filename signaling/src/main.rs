use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use server::Server;
use transport::UdpTransport;
use uuid::Uuid;

mod handlers;
mod message;
mod server;
mod transport;

type PeerId = Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

const PORT: u16 = 6969;

fn main() {
    let url = format!("0.0.0.0:{}", PORT);
    let transport = UdpTransport::new(&url).expect("failed to create transport layer");
    let mut server = Server::new(Box::new(transport));

    println!("INFO: listening to UDP datagrams at {}", url);
    server.listen();
}
