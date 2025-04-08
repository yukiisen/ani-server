use actix_web::{ web, HttpResponse };
use serde::Deserialize;
use sqlx::SqlitePool;

use ani_core::database::queries::get_top_animes;

use log::error;

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: u16,
    pub offset: u16,
}

pub async fn top_animes (query: web::Query<Query>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let top = get_top_animes(query.limit, query.offset, &pool).await;

    match top {
        Ok(top) => { HttpResponse::Ok().json(top) },
        Err(error) => {
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
