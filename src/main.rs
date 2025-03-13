use actix_web::{ App, HttpServer, web };
use anyhow::Result;

use ani_core::database::schema;
use ani_core::utils::config;

mod services;
mod models;
mod routes;
mod middlewares;
mod utils;

#[actix_web::main]
async fn main () -> Result<()> {
    env_logger::init();

    let config = config::load_config()?;
    let pool = schema::initialize(&config).await?;

    let config = web::Data::new(config);
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .app_data(pool.clone())
            .wrap(middlewares::cors_init())
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/v1")
                    .route("/anime/{mal_id}", web::get().to(routes::anime::get_anime))
                    .route("/anime/{mal_id}/episodes", web::get().to(routes::episodes::fetch_episodes))
                    .route("/search", web::get().to(routes::search::search_anime))
                    .route("/updates/{offset}", web::get().to(routes::updates::latest_updates))
                    .route("/synopsis/{mal_id}", web::get().to(routes::anime::fetch_synopsis))
            )
    });

    server
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
