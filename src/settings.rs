pub const SHORT_ID_ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

pub const SHORT_ID_LEN: u8 = 7;

pub struct ShortIdSettings {
    pub alphabet: Vec<u8>,
    pub id_len: u8,
}

pub struct RedisClusterSettings {
    pub server: String,
}

pub struct AppSettings {
    pub redis: RedisClusterSettings,
    pub short_id: ShortIdSettings,
}

impl ShortIdSettings {
    pub fn new() -> ShortIdSettings {
        // TODO: Get this from Redis
        ShortIdSettings {
            alphabet: Vec::from(SHORT_ID_ALPHABET),
            id_len: SHORT_ID_LEN,
        }
    }
}

impl RedisClusterSettings {
    pub fn new() -> RedisClusterSettings {
        RedisClusterSettings {
            server: String::from("redis://default:changeit@127.0.0.1/"),
        }
    }
}

impl AppSettings {
    pub fn new() -> AppSettings {
        AppSettings {
            redis: RedisClusterSettings::new(),
            short_id: ShortIdSettings::new(),
        }
    }
}
