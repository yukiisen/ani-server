use actix_cors::Cors;


// TODO: Reconfigure this after finishing all tests.

pub fn cors_init () -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
