use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use server::Server;
use transport::UdpTransport;
use uuid::Uuid;

mod config;
mod handlers;
mod message;
mod repositories;
mod server;
mod transport;

type PeerId = Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

#[tokio::main]
async fn main() {
    let url = format!("0.0.0.0:{}", PORT);
    let transport = UdpTransport::new(&url)
        .await
        .expect("failed to create transport layer");

    let mut server = Server::new(Box::new(transport));

    println!("INFO: listening to UDP datagrams at {}", url);
    server.listen().await;
}
