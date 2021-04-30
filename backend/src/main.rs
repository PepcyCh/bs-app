mod config;
mod database;
mod protocol;
mod server;

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};
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
    let config_json = include_str!("../config.json");
    let config: ServerConfig = serde_json::from_str(config_json).expect("Invalid config.json");
    assert!(config.validate(), "Invalid config.json");

    let database = web::Data::new(Database::new(config.db_url()).await.unwrap());

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
