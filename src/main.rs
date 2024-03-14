mod configuration;
mod endpoints;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use configuration::AppSettings;
use std::env;

// use crate::endpoints::redis_client::RedisClient;

// mod short_id;

// use redis::Commands;
// use settings::AppSettings;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::TemporaryRedirect().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = match AppSettings::new(env::vars().collect()) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read settings {:}", e),
    };

    println!(
        "Starting server on {:}:{:} ...",
        settings.host.ip_addr, settings.host.port
    );

    let app_data = settings.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(endpoints::health::health_checker_handler)
            .service(endpoints::generate::generate_short_url)
            // .service(web::scope("/").route("/", web::get().to(index)))
            .default_service(web::route().to(not_found))
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
