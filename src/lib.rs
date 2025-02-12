use actix_web::{ web, Responder };

async fn hello () -> impl Responder {
    "hello".customize().with_status(actix_web::http::StatusCode::NOT_FOUND)
}