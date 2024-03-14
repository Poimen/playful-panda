use crate::{configuration::AppSettings, endpoints::redis_client::retrieve_redirect_url};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/{short_code}")]
pub async fn redirect_short_code(
    settings: web::Data<AppSettings>,
    path: web::Path<String>,
) -> impl Responder {
    let short_code = path.into_inner();
    let validation_result = validate_short_code_request(&short_code);
    if validation_result.is_err() {
        return HttpResponse::BadRequest().json(validation_result.err());
    };

    let redirect_to = match retrieve_redirect_url(&settings, &short_code) {
        None => return HttpResponse::NotFound().body(""),
        Some(url) => url,
    };

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", redirect_to))
        .body("")
}

fn validate_short_code_request(short_url: &String) -> Result<bool, String> {
    if short_url.is_empty() {
        return Err(String::from("Url is missing"));
    }

    if short_url.len() > 250 {
        return Err(String::from("Url is too long"));
    }

    Ok(true)
}
