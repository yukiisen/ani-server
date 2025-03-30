use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

use ani_core::database::queries;

use queries::anime::get_anime_relations;

use log::error;

pub async fn anime_relations (mal_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let relations = get_anime_relations(*mal_id, &pool).await;

    match relations {
        Ok(relations) => HttpResponse::Ok().json(relations),
        Err(error) => {
            error!(target: "Database Error", "{error}"); 
            HttpResponse::InternalServerError().finish()
        }
    }   
}
