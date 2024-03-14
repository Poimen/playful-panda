mod configuration;
// mod settings;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use configuration::AppSettings;
use std::env;

// mod short_id;

// use redis::Commands;
// use settings::AppSettings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = match AppSettings::new(env::vars().collect()) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read settings {:}", e),
    };

    println!(
        "Starting server ... {:}:{:}",
        settings.host.ip_addr, settings.host.port
    );

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
    .bind((settings.host.ip_addr, settings.host.port))?
    .run()
    .await

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

#[get("/api/health")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json("healthy")
}
