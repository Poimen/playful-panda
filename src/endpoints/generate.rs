use super::short_id;
use crate::{
    configuration::AppSettings,
    endpoints::redis_client::{RedisClient, RedisClientError},
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct ShortCodeRequest {
    short_url: String,
    seconds: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct ShortCodeResponse {
    short_code: String,
}

pub async fn generate_short_url(
    State(settings): State<AppSettings>,
    State(redis_client): State<RedisClient>,
    Json(request): Json<ShortCodeRequest>,
) -> Result<Json<ShortCodeResponse>, (StatusCode, String)> {
    validate_short_code_request(&request.short_url).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    for _ in 0..settings.short_id.repeat_clash_len {
        let short_id = short_id::generate(&settings);

        match redis_client
            .set_if_not_exists(&short_id, &request.short_url, request.seconds)
            .await
        {
            Ok(()) => {
                return Ok(Json(ShortCodeResponse {
                    short_code: short_id,
                }));
            }
            Err(e) => match e {
                RedisClientError::ExpireKeyFailed(_) | RedisClientError::SetKeyFailed(_) => {
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, "".into()));
                }
                RedisClientError::KeyExists(_) => {}
                RedisClientError::GetValueFromKeyFailed(_) => panic!(),
                RedisClientError::ConnectionFailed(_) => panic!(),
            },
        }
    }

    Err((StatusCode::UNPROCESSABLE_ENTITY, "".into()))
}

fn validate_short_code_request(short_url: &str) -> Result<(), String> {
    if short_url.is_empty() {
        return Err(String::from("Url is missing"));
    }

    if short_url.len() > 250 {
        return Err(String::from("Url is too long"));
    }

    if !(short_url.starts_with("http://") || short_url.starts_with("https://")) {
        return Err(String::from("Not a http(s) url"));
    }

    Ok(())
}
