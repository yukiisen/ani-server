use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::web;

use sqlx::SqlitePool;

use ani_core::database::queries::get_anime_episodes;

pub async fn fetch_episodes (mal_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let eps = get_anime_episodes(*mal_id, &pool).await;

    match eps {
        Ok(eps) => { HttpResponse::Ok().json(eps) },
        Err(error) => {
            eprintln!("{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
