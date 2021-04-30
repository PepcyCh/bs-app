use crate::protocol::{LoginInfo, RegisterInfo};
use bson::doc;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

pub struct Database {
    _client: Client,
    users: Collection,
    _devices: Collection,
    _messages: Collection,
}

#[derive(Deserialize, Serialize)]
struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<bson::oid::ObjectId>,
    mail: String,
    name: String,
    password: String,
    devices: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Device {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<bson::oid::ObjectId>,
    id: String,
    name: String,
    info: String,
}

#[derive(Deserialize, Serialize)]
struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<bson::oid::ObjectId>,
    id: String,
    info: String,
    value: i32,
    alert: i32,
    lng: f64,
    lat: f64,
    timestamp: u64,
}

impl Database {
    pub async fn new(db_url: String) -> anyhow::Result<Self> {
        let options =
            ClientOptions::parse_with_resolver_config(&db_url, ResolverConfig::cloudflare())
                .await?;
        let client = Client::with_options(options)?;
        let database = client.database("bs_proj");
        let users = database.collection("users");
        let devices = database.collection("devices");
        let messages = database.collection("messages");

        Ok(Self {
            _client: client,
            users,
            _devices: devices,
            _messages: messages,
        })
    }

    pub async fn try_login(&self, info: LoginInfo) -> Result<(String, String), String> {
        let filter = doc! {
            "mail": info.mail
        };

        if let Ok(Some(doc)) = self.users.find_one(filter, None).await {
            if let Ok(user) = bson::from_bson::<User>(bson::Bson::Document(doc)) {
                return if user.password == info.password {
                    Ok((user.mail, user.name))
                } else {
                    Err("Wrong password".to_string())
                };
            }
        }
        Err("No such user".to_string())
    }

    pub async fn try_register(&self, info: RegisterInfo) -> Result<(), String> {
        let filter = doc! {
            "mail": &info.mail,
        };
        if let Ok(Some(_)) = self.users.find_one(filter, None).await {
            return Err("Duplicated mail address".to_string());
        }

        let filter = doc! {
            "name": &info.name,
        };
        if let Ok(Some(_)) = self.users.find_one(filter, None).await {
            return Err("Duplicated username".to_string());
        }

        let user = User {
            _id: None,
            mail: info.mail,
            name: info.name,
            password: info.password,
            devices: vec![],
        };
        let serialized_user = bson::to_bson(&user).unwrap();
        let doc = serialized_user.as_document().unwrap();
        if let Ok(_) = self.users.insert_one(doc.to_owned(), None).await {
            Ok(())
        } else {
            Err("Net error".to_string())
        }
    }
}
