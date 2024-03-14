#[derive(Debug, Clone)]
pub struct RedisClusterSettings {
    pub server: String,
}

impl RedisClusterSettings {
    pub fn new(server_url: &String) -> RedisClusterSettings {
        RedisClusterSettings {
            server: server_url.to_string(),
        }
    }
}
