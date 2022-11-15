use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::{debug, error, info, trace, warn};

mod settings;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let settings = settings::Settings::new().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new(&settings.logging_template))
    })
    .bind((settings.ip, settings.port))?
    .run()
    .await
}