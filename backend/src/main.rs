mod config;
mod database;
mod mqtt;
mod server;

use actix_web::{web, App, HttpServer};
use config::ServerConfig;
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Begin");
    mqtt::run_mqtt_broker();
    println!("MQTT broker is running");

    let config_json =
        std::fs::File::open("./config/server_cfg.json").expect("Server config json not found");
    let config: ServerConfig =
        serde_json::from_reader(&config_json).expect("Invalid server config json");
    assert!(config.validate(), "Invalid server config json");

    let database = web::Data::new(Database::new(config.db_url()).await.unwrap());
    println!("MongoDB is connected");

    mqtt::run_mqtt_subscriber(database.clone());
    println!("MQTT subscriber is running");
    println!("{}", config.addr());

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .configure(server::config)
    })
    .bind(config.addr())
    .expect("Failed to bind address")
    .run()
    .await
}
