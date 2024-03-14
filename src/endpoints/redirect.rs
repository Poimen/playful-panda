use crate::{configuration::AppSettings, endpoints::redis_client::retrieve_redirect_url};
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};

// #[get("/echo")]
// async fn echo(path: web::Path<String>) -> impl Responder {
//     let short_code = path.into_inner();

//     if short_code.is_empty() {
//         return HttpResponse::TemporaryRedirect().body("");
//     };

//     return Redirect::to(short_code);
// }

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

    // actix_web::web::Redirect::to("http://0.0.0.0/").see_other()

    HttpResponse::TemporaryRedirect()
        .append_header(("Location", redirect_to))
        .body("")
    // Redirect::to(redirect_to)
    // HttpResponse::TemporaryRedirect().
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
