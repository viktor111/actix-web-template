use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::{debug, error, info, trace, warn};

mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let settings = settings::Settings::new().unwrap();

    info!("Settings: {:?}", settings);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind((settings.ip, settings.port))?
    .run()
    .await
}