mod config;
mod database;
mod mqtt;
mod server;

use actix_files::NamedFile;
use actix_web::{App, HttpServer, Responder, get, web};
use config::ServerConfig;
use database::Database;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open("index.html")
}

#[get("/{filename}")]
async fn wasm(web::Path(filename): web::Path<String>) -> impl Responder {
    NamedFile::open(filename)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mqtt::run_mqtt_broker();

    let config_json = include_str!("../config/server_cfg.json");
    let config: ServerConfig = serde_json::from_str(config_json).expect("Invalid config.json");
    assert!(config.validate(), "Invalid config.json");

    let database = web::Data::new(Database::new(config.db_url()).await.unwrap());
    
    mqtt::run_mqtt_subscriber(database.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(wasm)
            .service(index)
            .configure(server::config)
    })
    .bind(config.addr())
    .expect("Failed to bind address")
    .run()
    .await
}