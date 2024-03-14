use crate::configuration::AppSettings;
use redis::{Commands, RedisError};

fn connect(settings: &AppSettings) -> Result<redis::Connection, RedisError> {
    let client: Result<redis::Client, redis::RedisError> =
        redis::Client::open(settings.redis.server.as_str());

    Ok(client?.get_connection()?)
}

pub fn store_short_code(
    settings: &AppSettings,
    short_id: &String,
    url_redirect: &String,
) -> Result<(), String> {
    let mut connection = match connect(settings) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    let lookup = format!("url_short:{short_id}");
    let exists = connection.exists(&lookup);

    if exists.is_err() || exists.unwrap() {
        return Err(String::from("Key exists already"));
    }

    let _: () = match connection.set(&lookup, url_redirect) {
        Err(e) => {
            return Err(String::from(
                e.detail().unwrap_or("Failed to set REDIS key"),
            ))
        }
        Ok(a) => a,
    };

    Ok(())
}
