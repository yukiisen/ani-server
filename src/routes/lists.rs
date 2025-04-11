use actix_web::{ web, HttpResponse };

use serde::{Deserialize, Serialize};
use serde_json::json;

use sqlx::prelude::FromRow;
use sqlx::SqlitePool;

use log::error;

fn http_error <T: std::error::Error>(error: T) -> HttpResponse {
     error!(target: "Database Error", "{error}"); 
     HttpResponse::InternalServerError().finish()
}

#[derive(FromRow, Serialize)]
pub struct List {
    id: u32,
    name: String
}

#[derive(Deserialize)]
pub struct NewList {
    name: String
}

#[derive(Deserialize)]
pub struct ListId {
    id: u32
}

#[derive(Deserialize)]
pub struct EntryIdentifier {
    id: u32
}

#[derive(Deserialize)]
pub struct Entry {
    mal_id: u32,
    list_id: u32
}


pub async fn get_lists (pool: web::Data<SqlitePool>) -> HttpResponse {
    let lists = sqlx::query_as::<_, List>("SELECT id, name FROM user_lists")
        .fetch_all(&(**pool))
        .await;

    lists.map_or_else(http_error,|lists| HttpResponse::Ok().json(lists))
}


pub async fn create_list (pool: web::Data<SqlitePool>, body: web::Json<NewList>) -> HttpResponse {
    let result = sqlx::query("INSERT INTO user_lists (name) VALUES ($1)")
        .bind(&body.name)
        .execute(&(**pool))
        .await;

    result.map_or_else(http_error, |res| {
        if res.rows_affected() == 1 {
            HttpResponse::Ok().json(json!({ "id": res.last_insert_rowid() }))
        }
        else {
            HttpResponse::InternalServerError().finish()
        }
    })
}

pub async fn delete_list (pool: web::Data<SqlitePool>, list: web::Query<ListId>) -> HttpResponse {
    let result = sqlx::query("DELETE FROM user_lists WHERE id = $1")
        .bind(list.id)
        .execute(&(**pool))
        .await;

    result.map_or_else(http_error, |res| {
        if res.rows_affected() == 1 {
            HttpResponse::Ok().finish()
        }
        else {
            HttpResponse::NotFound().finish()
        }
    })
}

pub async fn get_entries (pool: web::Data<SqlitePool>, list_id: web::Path<u32>) -> HttpResponse {
    let entries = sqlx::query_as::<_, (u32, u32, i64)>(
        "SELECT 
            list_entries.id
            animes.mal_id, 
            list_entries.added 
        FROM 
            list_entries 
        JOIN animes ON animes.id = list_entries.anime_id 
        WHERE list_entries.list_id = $1"
    )
        .bind(*list_id)
        .fetch_all(&(**pool))
        .await;

    let entries = entries.map(
        |mut vals| { 
            vals.sort_by_key(|val| val.2); // sort entries by date so that you don't have to do it!
            let vals: Vec<(u32, u32)> = vals.into_iter().map(|val| (val.0, val.1)).collect(); // remove added dates to reduce response size
            return vals;
        }
    );

    entries.map_or_else(http_error,|entries| HttpResponse::Ok().json(entries))
}

pub async fn remove_entry (pool: web::Data<SqlitePool>, entry: web::Query<EntryIdentifier>) -> HttpResponse {
    let result = sqlx::query("DELETE FROM list_entries WHERE id = $1")
        .bind(entry.id)
        .execute(&(**pool))
        .await;

    result.map_or_else(http_error, |res| {
        if res.rows_affected() == 1 {
            HttpResponse::Ok().finish()
        }
        else {
            HttpResponse::NotFound().finish()
        }
    })
}

pub async fn add_entry (pool: web::Data<SqlitePool>, entry: web::Json<Entry>) -> HttpResponse {
    let result = sqlx::query("INSERT INTO list_entries (list_id, anime_id) VALUES ($1, (SELECT id FROM animes WHERE mal_id = $2))")
        .bind(entry.list_id)
        .bind(entry.mal_id)
        .execute(&(**pool))
        .await;


    result.map_or_else(http_error, |res| {
        if res.rows_affected() == 1 {
            HttpResponse::Ok().finish()
        }
        else {
            HttpResponse::InternalServerError().finish()
        }
    })
}

pub async fn get_anime_lists (pool: web::Data<SqlitePool>) -> HttpResponse {
    let lists = sqlx::query_as::<_, List>("
        SELECT id, name 
        FROM user_lists 
        WHERE 
            EXISTS 
                (
                    SELECT list_entries.anime_id
                    FROM list_entries 
                    JOIN animes ON animes.id = list_entries.anime_id
                    WHERE animes.mal_id = $1 AND user_lists.id = list_entries.list_id
                )
        ")
        .fetch_all(&(**pool))
        .await;

    lists.map_or_else(http_error,|lists| HttpResponse::Ok().json(lists))
}
