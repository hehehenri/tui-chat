use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{transport::Transport, PeerAddr, PeerId};

type Peers = BTreeMap<PeerId, PeerAddr>;

pub struct Server {
    transport: Box<dyn Transport>,
    peers: Arc<Mutex<Peers>>,
}

impl Server {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let peers = Peers::default();
        let peers = Arc::new(Mutex::new(peers));

        Self { transport, peers }
    }
}

impl Server {
    pub fn listen(&self) {
        loop {
            let message = self.transport.receive();
            dbg!(message);
        }
    }
}
