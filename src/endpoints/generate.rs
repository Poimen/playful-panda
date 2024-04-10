use super::short_id;
use crate::{
    configuration::AppSettings,
    endpoints::redis_client::{RedisClient, RedisClientError},
};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct ShortCodeRequest {
    short_url: String,
    seconds: Option<u64>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct ShortCodeResponse {
    short_code: String,
}

#[post("/api/short-code")]
pub async fn generate_short_url(
    settings: web::Data<AppSettings>,
    redis: web::Data<RedisClient>,
    request: web::Json<ShortCodeRequest>,
) -> HttpResponse {
    let validation_result = validate_short_code_request(&request.short_url);
    if validation_result.is_err() {
        return HttpResponse::BadRequest().json(validation_result.err());
    };

    for _ in 0..settings.short_id.repeat_clash_len {
        let short_id = short_id::generate(&settings);

        match redis
            .set_if_not_exists(&short_id, &request.short_url, request.seconds)
            .await
        {
            Ok(()) => {
                return HttpResponse::Ok().json(ShortCodeResponse {
                    short_code: short_id,
                });
            }
            Err(e) => match e {
                RedisClientError::ExpireKeyFailed(_) | RedisClientError::SetKeyFailed(_) => {
                    return HttpResponse::InternalServerError().body("");
                }
                RedisClientError::KeyExists(_) => {}
                RedisClientError::GetValueFromKeyFailed(_) => panic!(),
                RedisClientError::ConnectionFailed(_) => panic!(),
            },
        }
    }

    HttpResponse::UnprocessableEntity().body("")
}

fn validate_short_code_request(short_url: &str) -> Result<bool, String> {
    if short_url.is_empty() {
        return Err(String::from("Url is missing"));
    }

    if short_url.len() > 250 {
        return Err(String::from("Url is too long"));
    }

    if !(short_url.starts_with("http://") || short_url.starts_with("https://")) {
        return Err(String::from("Not a http(s) url"));
    }

    Ok(true)
}
