use actix_web::{get, HttpResponse, Responder};

#[get("/api/health")]
pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

// pub fn get_health_endpoint() -> GetEndpointDef {
//     GetEndpointDef {
//         callback
//     }
// }
