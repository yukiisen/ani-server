use actix_web::web::route;
use actix_web::{ App, HttpServer, web };
use actix_files::Files;

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
                    .service(
                        web::scope("/anime")
                            .route("/{mal_id}", web::get().to(routes::anime::get_anime))
                            .route("/{mal_id}/episodes", web::get().to(routes::episodes::fetch_episodes))
                            .route("/{mal_id}/relations", web::get().to(routes::relations::anime_relations))
                            .route("/{mal_id}/recommendations", web::get().to(routes::recommendations::anime_recommendations))
                    )
                    .service(
                        web::scope("/notes")
                            .route("", web::get().to(routes::user::user_notes))
                            .route("/add", web::post().to(routes::user::insert_note))
                            .route("/edit", web::patch().to(routes::user::edit_note))
                            .route("/delete/{note_id}", web::delete().to(routes::user::remove_note))
                    )
                    .route("/search", web::get().to(routes::search::search_anime))
                    .route("/updates/{offset}", web::get().to(routes::updates::latest_updates))
                    .route("/synopsis/{mal_id}", web::get().to(routes::anime::fetch_synopsis))
            )
            .service(Files::new("/static", &config.images).show_files_listing())
    });

    server
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
