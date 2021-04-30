use crate::database::{Database, Message};
use actix_web::web;
use librumqttd::Config;
use rumqttc::{Client, Event, MqttOptions, Packet};

pub fn run_mqtt_broker() {
    let config: Config = confy::load_path("./config/mqtt_broker.toml")
        .expect("Invalid MQTT broker config");
    let mut broker = librumqttd::Broker::new(config);
    std::thread::spawn(move || {
        broker.start().expect("MQTT broker shut down due to error");
    });
}

pub fn run_mqtt_subscriber(db: web::Data<Database>) {
    let mut options = MqttOptions::new("mqtt_sub", "127.0.0.1", 1883);
    options.set_keep_alive(5);

    let (mut client, mut conn) = Client::new(options, 10);
    client.subscribe("testapp", rumqttc::QoS::AtMostOnce).unwrap();

    std::thread::spawn(move || {
        for msg in conn.iter() {
            if let Ok(Event::Incoming(Packet::Publish(msg))) = msg {
                let payloads = msg.payload;
                let msg: Message = serde_json::from_slice(&payloads).unwrap();
                if let Err(err) = async_std::task::block_on(db.insert_message(msg)) {
                    eprintln!("Failed to insert message, err = {}", err);
                }
            }
        }
    });
}