use std::sync::Arc;

use async_trait::async_trait;
use redis::Client;
use redis::Commands;
use redis::Connection;
use redis::SetOptions;

use crate::{config::RedisConfig, Peer};

#[derive(Debug)]
pub enum Error {
    FailedToStore(String),
    FailedToFetch(String),
    FailedToParse(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::FailedToStore(details) => format!("failed to store: {}", details),
            Self::FailedToFetch(details) => format!("failed to fetch: {}", details),
            Self::FailedToParse(details) => format!("failed to fetch: {}", details),
        }
    }
}

#[async_trait]
pub trait PeerRepository {
    async fn all(&self) -> Result<Vec<Peer>, Error>;
    async fn store(&self, peer: &Peer) -> Result<(), Error>;
}

pub struct RedisPeerRepository {
    client: Arc<Client>,
    config: RedisConfig,
}

impl RedisPeerRepository {
    pub fn new(client: &Arc<Client>, config: &RedisConfig) -> Self {
        Self {
            client: client.clone(),
            config: config.clone(),
        }
    }
}

impl RedisPeerRepository {
    fn connection(&self) -> Result<Connection, Error> {
        self.client
            .get_connection()
            .map_err(|err| Error::FailedToFetch(err.to_string()))
    }
}

#[async_trait]
impl PeerRepository for RedisPeerRepository {
    async fn all(&self) -> Result<Vec<Peer>, Error> {
        let mut con = self.connection()?;

        let keys: Vec<String> = con
            .scan()
            .map_err(|err| Error::FailedToFetch(err.to_string()))?
            .filter(|key: &String| key.starts_with("peer:"))
            .collect();

        let mut peers: Vec<Peer> = Vec::with_capacity(keys.len());

        // TODO: this looks REALLY bad. Isn't there any way to get all
        // values that match a given key patter with native redis commands?
        for key in keys {
            let peer: String = con
                .get(key)
                .map_err(|err| Error::FailedToFetch(err.to_string()))?;

            let peer: Peer =
                serde_json::from_str(&peer).map_err(|err| Error::FailedToParse(err.to_string()))?;

            peers.push(peer);
        }

        Ok(peers)
    }

    async fn store(&self, peer: &Peer) -> Result<(), Error> {
        let mut con = self.connection()?;

        let key = format!("peer:{}", peer.id);

        let peer =
            serde_json::to_string(&peer).map_err(|err| Error::FailedToParse(err.to_string()))?;

        let options = SetOptions::default().with_expiration(self.config.ttl);

        con.set_options(key, peer, options)
            .map_err(|err| Error::FailedToStore(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use crate::config::Config;

    use super::*;

    fn random_addr() -> SocketAddr {
        let mut rng = rand::thread_rng();

        let ip_addr = IpAddr::V4(Ipv4Addr::new(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        ));

        let port = rand::thread_rng().gen_range(1024..65535);

        SocketAddr::new(ip_addr, port)
    }

    fn peer() -> Peer {
        Peer::new(random_addr())
    }

    fn redis_client() -> Client {
        let config = Config::from_env();
        redis::Client::open(config.redis.url).expect("failed to create redis test client")
    }

    fn connection() -> Connection {
        redis_client()
            .get_connection()
            .expect("failed to create redis test connection")
    }

    async fn repository() -> RedisPeerRepository {
        let config = Config::from_env();
        let client = redis_client();

        RedisPeerRepository::new(&Arc::new(client), &config.redis)
    }

    #[tokio::test]
    async fn it_stores_peer_into_redis() {
        let peer = peer();
        let repo = repository().await;

        repo.store(&peer).await.expect("failed to store peer");

        let key = format!("peer:{}", peer.id);
        let stored_peer: String = connection().get(key).expect("key not found");
        let stored_peer: Peer = serde_json::from_str(&stored_peer).expect("failed to parse peer");

        assert!(stored_peer == peer)
    }
}
