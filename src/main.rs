mod configuration;
mod endpoints;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use configuration::AppSettings;
use endpoints::redis_client::RedisClient;
use std::env;
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    _ = dotenvy::from_path(Path::new(".env.development"));
    env_logger::init();

    let settings = match AppSettings::new(env::vars().collect()) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read settings {:}", e),
    };

    let redis_client = RedisClient::new(&settings).await.unwrap();

    println!(
        "Starting server on {:}:{:} ...",
        settings.host.ip_addr, settings.host.port
    );

    let app_data = settings.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .service(endpoints::health::health_checker_handler)
            .service(endpoints::generate::generate_short_url)
            .service(endpoints::redirect::redirect_short_code)
            .wrap(Logger::default())
    })
    .bind((settings.host.ip_addr, settings.host.port))?
    .run()
    .await
}
