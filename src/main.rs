mod settings;
mod short_id;

use redis::Commands;
use settings::AppSettings;

fn main() {
    let url: String = String::from("https://www.google.com");
    let settings = AppSettings::new();
    let short_id = short_id::generate(&settings.short_id);

    println!("short-id: {}", short_id);

    let client = redis::Client::open(settings.redis.server.as_str());
    let mut con = client.unwrap().get_connection().unwrap();

    let lookup = format!("url_short:{short_id}");
    let _: () = con.set(&lookup, url).unwrap();

    let res: bool = con.exists(&lookup).unwrap();

    let new_id: String = con.get(&lookup).unwrap();
    println!("short-id: {} {}", new_id, res);
}
