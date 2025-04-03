use actix_web::{ web, HttpResponse };
use sqlx::SqlitePool;

use ani_core::database::queries::get_top_animes;

use log::error;

pub async fn top_animes (limit: web::Path<u16>, offset: web::Path<u16>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let top = get_top_animes(*limit, *offset, &pool).await;

    match top {
        Ok(top) => { HttpResponse::Ok().json(top) },
        Err(error) => {
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
