use actix_web::Responder;
use actix_web::web;
use actix_web::HttpResponse;

use ani_core::database::queries;

use queries::anime::get_anime_by_id;
use queries::metadata::get_synopsis;

use serde::Deserialize;
use std::collections::HashMap;
use sqlx::SqlitePool;

pub async fn get_anime (mal_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let anime = get_anime_by_id(*mal_id, &pool).await;

    match anime {
        Ok(Some(anime)) => { HttpResponse::Ok().json(anime) },
        Ok(None) => { HttpResponse::NotFound().finish() },
        Err(error) => { 
            eprintln!("{error}");
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[derive(Deserialize)]
pub struct SynQuery {
    pub lang: String
}

pub async fn fetch_synopsis (query: web::Query<SynQuery>, mal_id: web::Path<u32>, pool: web::Data<SqlitePool>) -> impl Responder {
    let synopsis = get_synopsis(*mal_id, &query.lang, &pool).await;

    match synopsis {
        Ok(syn) => { HttpResponse::Ok().json(HashMap::from([ ( "synopsis", syn ) ])) },
        Err(error) => { 
            eprintln!("{error}");
            HttpResponse::InternalServerError().finish() 
        },
    }
}
