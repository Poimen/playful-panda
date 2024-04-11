mod app_state;
mod configuration;
mod endpoints;

use app_state::AppState;
use axum::routing::{get, post};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
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

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app_state = AppState::new().await?;

    let app_routes = Router::new()
        .route("/health", get(endpoints::health::health))
        .route(
            "/api/short-code",
            post(endpoints::generate::generate_short_url),
        )
        .route("/:short_code", get(endpoints::redirect::redirect))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(
            ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_millis(
                app_state.settings.host.req_timeout_ms,
            ))),
        )
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
}
