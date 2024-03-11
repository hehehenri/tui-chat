use std::sync::{Arc, Mutex};

use crate::{handlers::join, message::Message, transport::Transport, Peer};

// TODO: use something like R2
pub type Peers = Vec<Peer>;

#[derive(Default)]
pub struct Context {
    pub peers: Arc<Mutex<Peers>>,
}

pub struct Server {
    transport: Box<dyn Transport>,
    context: Context,
}

impl Server {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let context = Context::default();

        Self { transport, context }
    }

    pub fn handle(&mut self, message: Message) {
        match message {
            Message::Join(join) => join::handle(join, &self.context, &self.transport),
        }
    }

    pub fn listen(&mut self) {
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

            self.handle(message)
        }
    }
}
