use std::sync::Arc;

use redis::Client;

use crate::{
    config::Config, handlers::join, message::Message, repositories::Repositories,
    transport::Transport,
};

pub struct Context {
    pub redis_client: Arc<Client>,
    pub config: Config,
    pub repositories: Repositories,
}

impl Context {
    pub fn new() -> Self {
        let config = Config::from_env();
        let redis =
            redis::Client::open(config.redis.url.clone()).expect("failed to connect to redis");
        let redis = Arc::new(redis);

        let repositories = Repositories::new(&redis, &config);

        Self {
            config,
            repositories,
            redis_client: redis,
        }
    }
}

pub struct Server {
    transport: Box<dyn Transport>,
    context: Context,
}

impl Server {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let context = Context::new();

        Self { transport, context }
    }

    async fn handle(&mut self, message: Message) {
        match message {
            Message::Join(join) => join::handle(join, &self.transport, &self.context).await,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let incoming_message = match self.transport.receive().await {
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

            self.handle(message).await
        }
    }
}
