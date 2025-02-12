use actix_web::{ web, Responder };

pub async fn hello () -> impl Responder {
    "hello"
}