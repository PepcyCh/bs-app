use bson::doc;
use common::{
    request::{
        CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceRequest, FetchMessageListRequest,
        LoginRequest, ModifyDeviceRequest, RegisterRequest, RemoveDeviceRequest,
    },
    response::{DeviceInfo, MessageInfo},
};
use futures::StreamExt;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

pub struct Database {
    users: Collection,
    devices: Collection,
    messages: Collection,
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
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<bson::oid::ObjectId>,
    id: String,
    info: String,
    value: u32,
    alert: bool,
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
            users,
            devices,
            messages,
        })
    }

    pub async fn login(&self, info: LoginRequest) -> Result<(String, String), String> {
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

    pub async fn register(&self, info: RegisterRequest) -> Result<(), String> {
        let filter = doc! {
            "mail": &info.mail,
        };
        if let Some(_) = self.users.find_one(filter, None).await.unwrap() {
            return Err("Duplicated mail address".to_string());
        }

        let filter = doc! {
            "name": &info.name,
        };
        if let Some(_) = self.users.find_one(filter, None).await.unwrap() {
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
            Err("Unknown error".to_string())
        }
    }

    pub async fn insert_message(&self, msg: Message) -> Result<(), String> {
        let serialized_msg = bson::to_bson(&msg).unwrap();
        let doc = serialized_msg.as_document().unwrap();
        if let Ok(_) = self.messages.insert_one(doc.to_owned(), None).await {
            Ok(())
        } else {
            Err("Unknown error".to_string())
        }
    }

    pub async fn create_device(&self, info: CreateDeviceRequest) -> Result<(), String> {
        let filter = doc! {
            "mail": info.mail.clone(),
        };
        if let None = self.users.find_one(filter, None).await.unwrap() {
            return Err("User doesn't exist".to_string());
        }

        let filter = doc! {
            "id": info.id.clone()
        };
        if let None = self.devices.find_one(filter, None).await.unwrap() {
            let dev = Device {
                _id: None,
                id: info.id.clone(),
                name: info.id.clone(),
                info: "".to_string(),
            };
            let serialized_dev = bson::to_bson(&dev).unwrap();
            let doc = serialized_dev.as_document().unwrap();
            if let Err(_) = self.devices.insert_one(doc.to_owned(), None).await {
                return Err("Unknown error".to_string());
            }
        }

        let query = doc! {
            "mail": info.mail
        };
        let update = doc! {
            "$push": {
                "devices": info.id.clone(),
            }
        };
        if let Ok(_) = self.users.update_one(query, update, None).await {
            Ok(())
        } else {
            Err("Unknown error".to_string())
        }
    }

    pub async fn remove_device(&self, info: RemoveDeviceRequest) -> Result<(), String> {
        let filter = doc! {
            "mail": info.mail.clone(),
        };
        if let None = self.users.find_one(filter, None).await.unwrap() {
            return Err("User doesn't exist".to_string());
        }

        let filter = doc! {
            "mail": info.mail.clone(),
            "devices": {
                "$elemMatch": info.id.clone()
            }
        };
        if let None = self.users.find_one(filter, None).await.unwrap() {
            return Err("Device doesn't exist".to_string());
        }

        let query = doc! {
            "mail": info.mail
        };
        let update = doc! {
            "$pull": {
                "devices": info.id.clone(),
            }
        };
        if let Ok(_) = self.users.update_one(query, update, None).await {
            Ok(())
        } else {
            Err("Unknown error".to_string())
        }
    }

    pub async fn modify_device(&self, info: ModifyDeviceRequest) -> Result<(), String> {
        let filter = doc! {
            "id": info.id.clone(),
        };
        let device = self.devices.find_one(filter, None).await.unwrap();
        if device.is_none() {
            return Err("Device doesn't exist".to_string());
        }

        let query = doc! {
            "id": info.id,
        };
        let update = doc! {
            "$set": {
                "name": info.name,
                "info": info.info,
            }
        };
        if let Ok(_) = self.devices.update_one(query, update, None).await {
            Ok(())
        } else {
            Err("Unknown error".to_string())
        }
    }

    pub async fn fetch_device(
        &self,
        info: FetchDeviceRequest,
    ) -> Result<(String, String, String), String> {
        let filter = doc! {
            "id": info.id
        };
        let device = self.devices.find_one(filter, None).await.unwrap();
        if device.is_none() {
            return Err("Device doesn't exist".to_string());
        }
        let device: Device = bson::from_bson(bson::Bson::Document(device.unwrap())).unwrap();
        Ok((device.id, device.name, device.info))
    }

    pub async fn fetch_device_list(
        &self,
        info: FetchDeviceListRequest,
    ) -> Result<Vec<DeviceInfo>, String> {
        let filter = doc! {
            "mail": info.mail.clone(),
        };
        let user = self.users.find_one(filter, None).await.unwrap();
        if user.is_none() {
            return Err("User doesn't exist".to_string());
        }
        let user: User = bson::from_bson(bson::Bson::Document(user.unwrap())).unwrap();

        let mut devices = Vec::with_capacity(user.devices.len());
        for id in &user.devices {
            let filter = doc! {
                "id": id.clone()
            };
            if let Some(dev) = self.devices.find_one(filter, None).await.unwrap() {
                let dev: Device = bson::from_bson(bson::Bson::Document(dev)).unwrap();

                let count_filter = doc! {
                    "id": dev.id.clone(),
                };
                let message_count = self
                    .messages
                    .count_documents(count_filter, None)
                    .await
                    .unwrap() as u32;
                let count_filter = doc! {
                    "id": dev.id.clone(),
                    "alert": true,
                };
                let alert_message_count = self
                    .messages
                    .count_documents(count_filter, None)
                    .await
                    .unwrap() as u32;

                let dev = DeviceInfo {
                    id: dev.id,
                    name: dev.name,
                    message_count,
                    alert_message_count,
                };
                devices.push(dev);
            } else {
                return Err("Device doesn't exist".to_string());
            }
        }

        Ok(devices)
    }

    pub async fn fetch_message_list(
        &self,
        info: FetchMessageListRequest,
    ) -> Result<Vec<MessageInfo>, String> {
        let filter = doc! {
            "id": info.id.clone(),
        };
        if let None = self.users.find_one(filter, None).await.unwrap() {
            return Err("Device doesn't exist".to_string());
        }

        let filter = doc! {
            "id": info.id.clone(),
            "timestamp": {
                "$gte": info.start_timestamp,
                "$lte": info.end_timestamp,
            }
        };
        let mut cursor = self.messages.find(filter, None).await.unwrap();
        let mut messages = vec![];
        while let Some(msg) = cursor.next().await {
            let msg: Message = bson::from_bson(bson::Bson::Document(msg.unwrap())).unwrap();
            let msg = MessageInfo {
                id: msg.id,
                info: msg.info,
                value: msg.value,
                alert: msg.alert,
                lng: msg.lng,
                lat: msg.lat,
                timestamp: msg.timestamp,
            };
            messages.push(msg);
        }

        Ok(messages)
    }
}

impl Message {
    pub fn new(
        id: String,
        info: String,
        value: u32,
        alert: bool,
        lng: f64,
        lat: f64,
        timestamp: u64,
    ) -> Self {
        Self {
            _id: None,
            id,
            info,
            value,
            alert,
            lng,
            lat,
            timestamp,
        }
    }
}
