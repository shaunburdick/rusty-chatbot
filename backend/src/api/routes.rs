use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::db::db::DB;
use crate::api::error::HttpError;
use crate::api::response::JsonApiResponse;

#[get("/voices")]
async fn voices_find_all(db: web::Data<DB>) -> Result<HttpResponse, HttpError> {
    let voices = db.get_voices(false).await?;
    Ok(HttpResponse::Ok().json(JsonApiResponse::success(voices, None)))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(voices_find_all);
}
