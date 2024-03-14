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

pub fn retrieve_redirect_url(settings: &AppSettings, short_id: &String) -> Option<String> {
    let mut connection = match connect(settings) {
        Ok(c) => c,
        Err(e) => return None,
    };

    let lookup = format!("url_short:{short_id}");

    match connection.get(&lookup) {
        Ok(url) => Some(url),
        Err(_) => None,
    }

    // let url: String = String::from("https://www.google.com");
    // let settings = AppSettings::new();
    // let short_id = short_id::generate(&settings.short_id);

    // println!("short-id: {}", short_id);

    // let client = redis::Client::open(settings.redis.server.as_str());
    // let mut con = client.unwrap().get_connection().unwrap();

    // let lookup = format!("url_short:{short_id}");
    // let _: () = con.set(&lookup, url).unwrap();

    // let res: bool = con.exists(&lookup).unwrap();

    // let new_id: String = con.get(&lookup).unwrap();
    // println!("short-id: {} {}", new_id, res);
}
