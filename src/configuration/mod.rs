mod host_settings;
mod redis_settings;
mod short_id_settings;

use host_settings::HostSettings;
use redis_settings::RedisSettings;
use short_id_settings::ShortIdSettings;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub host: HostSettings,
    pub redis: RedisSettings,
    pub short_id: ShortIdSettings,
}

impl AppSettings {
    pub fn new(args: HashMap<String, String>) -> Result<AppSettings, String> {
        let redis_server_url = match args.get("REDIS_SERVER_URL") {
            Some(x) => x,
            None => return Err(String::from("Redis Server url required")),
        };

        Ok(AppSettings {
            host: HostSettings::new(args.get("HOST_IP"), args.get("HOST_PORT")),
            redis: RedisSettings::new(redis_server_url),
            short_id: ShortIdSettings::new(
                args.get("ALPHABET"),
                args.get("SHORT_ID_LENGTH"),
                args.get("SHORT_ID_REPEAT_CLASh_LENGTH"),
            ),
        })
    }
}
