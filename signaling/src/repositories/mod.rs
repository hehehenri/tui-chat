use self::peer_repository::{PeerRepository, RedisPeerRepository};
use crate::config::Config;
use redis::Client;
use std::sync::Arc;

pub mod peer_repository;

pub struct Repositories {
    pub peer_repository: Box<dyn PeerRepository>,
}

impl Repositories {
    pub fn new(redis: &Arc<Client>, config: &Config) -> Self {
        let peer_repository = RedisPeerRepository::new(redis, &config.redis);

        Self {
            peer_repository: Box::new(peer_repository),
        }
    }
}
