use uuid::Uuid;

use crate::{transport::TransportMessage, Peer, PeerId};

#[derive(Debug)]
struct Join(Peer);

impl From<TransportMessage> for Join {
    fn from(message: TransportMessage) -> Self {
        let peer = Peer::new(message.addr);

        Self(peer)
    }
}

#[derive(Debug)]
struct StartConnection {
    peer_id: PeerId,
}

impl TryFrom<&[u8]> for StartConnection {
    type Error = DecodingError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let peer_id = Uuid::from_slice(bytes).map_err(|err| invalid_message(&err.to_string()))?;

        let message = StartConnection { peer_id };
        Ok(message)
    }
}

#[derive(Debug)]
struct ConnectionConfirmation {
    client: PeerId,
    peer: PeerId,
}

impl TryFrom<&[u8]> for ConnectionConfirmation {
    type Error = DecodingError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let peer_id = bytes[..20].to_vec();
        let remaining_bytes = &bytes[20..];

        if !remaining_bytes.is_empty() {
            return Err(invalid_message(
                "invalid connection_confirmation message: got too many bytes",
            ));
        }

        let message = ConnectionConfirmation { peer_id };
        Ok(message)
    }
}

#[derive(Debug)]
pub enum Message {
    Join(Join),
    StartConnection(StartConnection),
    ConnectionConfirmation(ConnectionConfirmation),
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

fn invalid_message(details: &str) -> DecodingError {
    DecodingError::InvalidMessage(details.to_string())
}

impl TryFrom<TransportMessage> for Message {
    type Error = DecodingError;

    fn try_from(message: TransportMessage) -> Result<Self, Self::Error> {
        match message.header {
            1 => Ok(Join::from(message)),
            2 => StartConnection::try_from(message),
            3 => ConnectionConfirmation::try_from(message),
            _ => {
                let details = invalid_message(&format!("invalid header {}", message.header));
                Err(details)
            }
        }
    }
}
