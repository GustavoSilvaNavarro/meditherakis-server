use actix_web::{get, web::ServiceConfig, HttpResponse};

#[get("/healthz")]
async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

#[get("/check")]
async fn check_service() -> HttpResponse {
    HttpResponse::Ok().body("Service Health checked ✅")
}

pub fn monitoring_routes(config: &mut ServiceConfig) {
    config.service(health_check);
    config.service(check_service);
}
