use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{message::Message, transport::Transport, Peer, PeerId};

type Peers = BTreeMap<PeerId, Peer>;

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
            let incoming_message = match self.transport.receive() {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprintln!("failed to receive message: {}", err.to_string());
                    continue;
                }
            };

            let message = match Message::try_from(incoming_message) {
                Ok(message) => message,
                Err(err) => {
                    eprintln!("failed to decode message: {}", err.to_string());
                    continue;
                }
            };

            dbg!(message);
        }
    }

    fn handle(&mut self, message: Message) {
        match message {
            Message::Join(message) => todo!(),
            _ => todo!(),
        }
    }
}
