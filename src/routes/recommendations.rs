use actix_web::web;
use actix_web::HttpResponse;

use ani_core::database::queries;
use queries::get_anime_recommendations;

use sqlx::SqlitePool;
use log::error;


pub async fn anime_recommendations (mal_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> HttpResponse {
    let recommended = get_anime_recommendations(*mal_id, &pool).await;

    match recommended {
        Ok(recs) => {
            HttpResponse::Ok().json(recs)
        },
        Err(error) => {
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
