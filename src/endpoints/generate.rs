use super::short_id;
use crate::{configuration::AppSettings, endpoints::redis_client::store_short_code};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ShortCodeRequest {
    short_url: String,
}

#[post("/api/short-code")]
pub async fn generate_short_url(
    settings: web::Data<AppSettings>,
    request: web::Json<ShortCodeRequest>,
) -> HttpResponse {
    let validation_result = validate_short_code_request(&request.short_url);
    if validation_result.is_err() {
        return HttpResponse::BadRequest().json(validation_result.err());
    };

    let short_id = short_id::generate(&settings);

    match store_short_code(&settings, &short_id, &request.short_url) {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(e) => HttpResponse::UnprocessableEntity().json(e),
    }
}

fn validate_short_code_request(short_url: &String) -> Result<bool, String> {
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
