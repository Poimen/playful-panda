mod app_state;
mod configuration;
mod endpoints;

use app_state::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::error::Error;
use std::path::Path;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    _ = dotenvy::from_path(Path::new(".env.development"));
    _ = dotenvy::from_path(Path::new(".env"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "url_short=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState::new().await?;

    let app_routes = Router::new()
        .route("/health", get(endpoints::health::health))
        .route(
            "/api/short-code",
            post(endpoints::generate::generate_short_url),
        )
        .route("/:short_code", get(endpoints::redirect::redirect))
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind((
        app_state.settings.host.ip_addr,
        app_state.settings.host.port,
    ))
    .await
    .expect("Could not initialize TcpListener");

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .expect("Could not convert listener address to local address")
    );

    axum::serve(listener, app_routes)
        .await
        .expect("Could not successfully create server");

    Ok(())

    // let app_data = settings.clone();
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(web::Data::new(app_data.clone()))
    //         .app_data(web::Data::new(redis_client.clone()))
    //         .service(endpoints::health::health_checker_handler)
    //         .service(endpoints::generate::generate_short_url)
    //         .service(endpoints::redirect::redirect_short_code)
    //         .wrap(Logger::default())
    // })
    // .bind((settings.host.ip_addr, settings.host.port))?
    // .run()
    // .await
}
