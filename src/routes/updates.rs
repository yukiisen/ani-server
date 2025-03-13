use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;

use ani_core::database::queries::anime::get_latest_updates;
use sqlx::SqlitePool;

pub async fn latest_updates (offset: web::Path<u16>, pool: web::Data<SqlitePool>) -> impl Responder {
    let updates = get_latest_updates(*offset, &pool).await;

    match updates {
        Ok(updates) => {
            HttpResponse::Ok().json(updates)
        },
        Err(error) => {
            eprintln!("{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
