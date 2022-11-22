use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use jwt_simple::reexports::serde_json::json;

#[get("/info")]
pub async fn info(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": data.version,
        "protocol_version": "0.4.1",
        "last_update": *data.last_update.read(),
    }))
}
