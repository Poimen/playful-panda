#[derive(Debug, Clone)]
pub struct RedisSettings {
    pub server: String,
}

impl RedisSettings {
    pub fn new(server_url: &String) -> RedisSettings {
        RedisSettings {
            server: server_url.to_string(),
        }
    }
}
