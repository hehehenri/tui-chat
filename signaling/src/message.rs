use crate::{transport::TransportMessage, Peer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Join(Peer);

impl From<TransportMessage> for Join {
    fn from(message: TransportMessage) -> Self {
        let peer = Peer::new(message.addr);

        Self(peer)
    }
}

#[derive(Debug, Deserialize)]
pub enum Message {
    Join(Join),
}

pub enum DecodingError {
    InvalidMessage(String),
}

impl ToString for DecodingError {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidMessage(details) => format!("invalid message: {}", details),
        }
    }
}

impl TryFrom<TransportMessage> for Message {
    type Error = DecodingError;

    fn try_from(message: TransportMessage) -> Result<Self, Self::Error> {
        serde_json::from_str(&message.content)
            .map_err(|err| DecodingError::InvalidMessage(err.to_string()))
    }
}
