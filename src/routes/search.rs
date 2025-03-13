use actix_web::{web, HttpResponse, Responder};
use ani_core::database::queries::search::search_anime_by_name;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search_anime (query: web::Query<SearchQuery>, pool: web::Data<SqlitePool>) -> impl Responder {
    let rows = search_anime_by_name(&query.q, &pool).await;
    
    match rows {
        Ok(rows) => {
            HttpResponse::Ok().json(rows)
        },
        Err(error) => {
            eprintln!("{}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}
