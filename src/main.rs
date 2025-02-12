use actix_web::{
    web,
    App,
    HttpServer,
    Responder
};

#[actix_web::main]
async fn main () -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
    });

    server
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}