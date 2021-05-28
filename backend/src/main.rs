mod config;
mod database;
mod mqtt;
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

#[get("/snippets/{crate_name}/build/{filename}")]
async fn mwc_js(web::Path((crate_name, filename)): web::Path<(String, String)>) -> impl Responder {
    NamedFile::open(format!("snippets/{}/build/{}", crate_name, filename))
}

#[get("/snippets/{crate_name}/src/utils/{filename}")]
async fn comp_js(web::Path((crate_name, filename)): web::Path<(String, String)>) -> impl Responder {
    NamedFile::open(format!("snippets/{}/src/utils/{}", crate_name, filename))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Begin");
    mqtt::run_mqtt_broker();
    println!("MQTT broker is running");

    let config_json = include_str!("../config/server_cfg.json");
    let config: ServerConfig = serde_json::from_str(config_json).expect("Invalid config.json");
    assert!(config.validate(), "Invalid config.json");

    let database = web::Data::new(Database::new(config.db_url()).await.unwrap());
    println!("MongoDB is connected");

    mqtt::run_mqtt_subscriber(database.clone());
    println!("MQTT subscriber is running");

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(wasm)
            .service(mwc_js)
            .service(comp_js)
            .service(index)
            .configure(server::config)
    })
    .bind(config.addr())
    .expect("Failed to bind address")
    .run()
    .await
}
