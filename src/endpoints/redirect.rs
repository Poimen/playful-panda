use super::redis_client::RedisClientError;
use crate::endpoints::redis_client::RedisClient;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;

const DEFAULT_CACHE_CONTROL_HEADER_VALUE: &str =
    "public, max-age=300, s-maxage=300, stale-while-revalidate=300, stale-if-error=300";

pub async fn redirect(
    State(redis_client): State<RedisClient>,
    Path(short_code): Path<String>,
) -> Result<Response, StatusCode> {
    _ = validate_short_code_request(&short_code).map_err(|_| StatusCode::BAD_REQUEST)?;

    let redirect_to = redis_client.get(&short_code).await.map_err(|e| match e {
        RedisClientError::ConnectionFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        RedisClientError::GetValueFromKeyFailed(_) => StatusCode::NOT_FOUND,
        RedisClientError::KeyExists(_) => panic!(),
        RedisClientError::SetKeyFailed(_) => panic!(),
        RedisClientError::ExpireKeyFailed(_) => panic!(),
    })?;

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", redirect_to)
        .header("Cache-Control", DEFAULT_CACHE_CONTROL_HEADER_VALUE)
        .body(Body::empty())
        .expect("Should always construct response"))
}

fn validate_short_code_request(short_url: &str) -> Result<(), String> {
    if short_url.is_empty() {
        return Err(String::from("Url is missing"));
    }

    if short_url.len() > 250 {
        return Err(String::from("Url is too long"));
    }

    Ok(())
}
