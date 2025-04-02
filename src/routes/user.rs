use actix_web::web;
use actix_web::HttpResponse;

use sqlx::SqlitePool;

use ani_core::models::EditedNote;
use ani_core::models::Note;
use ani_core::database::queries;

use queries::get_notes;
use queries::set_note;
use queries::add_note;
use queries::delete_note;

use log::error;

pub async fn user_notes (pool: web::Data<SqlitePool>, mal_id: web::Path<u32>) -> HttpResponse {
    let notes = get_notes(&mal_id, &pool).await;

    match notes {
        Ok(notes) => { HttpResponse::Ok().json(notes) },
        Err(error) => { 
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert_note (pool: web::Data<SqlitePool>, note: web::Json<Note>) -> HttpResponse {
    let result = add_note(&note, &pool).await;

    match result {
        Ok(_) => { HttpResponse::Ok().finish() },
        Err(error) => { 
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn edit_note (pool: web::Data<SqlitePool>, note: web::Json<EditedNote>) -> HttpResponse {
    let result = set_note(note.id, &note.note, &pool).await;

    match result {
        Ok(_) => { HttpResponse::Ok().finish() },
        Err(error) => { 
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn remove_note (pool: web::Data<SqlitePool>, note_id: web::Path<u32>) -> HttpResponse {
    let result = delete_note(*note_id, &pool).await;

    match result {
        Ok(_) => { HttpResponse::Ok().finish() },
        Err(error) => { 
            error!(target: "Database Error", "{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
