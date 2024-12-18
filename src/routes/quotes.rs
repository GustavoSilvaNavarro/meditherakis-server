use crate::entities::quotes;
use crate::schemas::quotes::NewQuotePayload;
use actix_web::{
    post,
    web::{Data, Json, ServiceConfig},
    HttpResponse,
};
use log::error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde_json::json;

#[post("/new-quote")]
async fn create_quote(
    db: Data<DatabaseConnection>,
    payload: Json<NewQuotePayload>,
) -> HttpResponse {
    let new_quote = quotes::ActiveModel {
        quote: Set(payload.quote.clone()),
        ..Default::default()
    };

    match new_quote.insert(db.get_ref()).await {
        Ok(created_quote) => HttpResponse::Created().json(created_quote),
        Err(err) => {
            error!("Error creating new quote: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({ "status": "error", "message": format!("{:?}", err) }))
        }
    }
}

pub fn quote_routes(config: &mut ServiceConfig) {
    config.service(create_quote);
}
