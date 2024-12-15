use actix_web::{
    post,
    web::{Json, ServiceConfig},
    Responder,
};
use serde_json::json;

#[post("/new-quote")]
async fn create_quote() -> impl Responder {
    Json(json!({ "msg": "Hello Gustavo from Rust" }))
}

pub fn quote_routes(config: &mut ServiceConfig) {
    config.service(create_quote);
}
