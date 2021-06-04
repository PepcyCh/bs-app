use anyhow::{bail, Context};
use bson::doc;
use chrono::Utc;
use common::{
    request::{
        CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceProfileRequest, FetchDeviceRequest,
        FetchMessageListRequest, LoginRequest, ModifyDeviceRequest, RegisterRequest,
        RemoveDeviceRequest,
    },
    response::{DeviceInfo, MessageInfo},
};
use futures::StreamExt;
use mongodb::{
    options::{ClientOptions, FindOneOptions, FindOptions, ResolverConfig},
    Client, Collection,
};
use serde::{Deserialize, Serialize};

pub struct Database {
    users: Collection,
    devices: Collection,
    messages: Collection,
    login_records: Collection,
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
    value: i32,
    alert: bool,
    lng: f64,
    lat: f64,
    timestamp: i64,
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
        let login_records = database.collection("login_records");

        Ok(Self {
            users,
            devices,
            messages,
            login_records,
        })
    }

    pub async fn login(&self, info: LoginRequest) -> anyhow::Result<(String, String, String)> {
        let filter = doc! {
            "mail": info.mail
        };

        if let Some(doc) = self.users.find_one(filter, None).await.context("error-net")? {
        let user = bson::from_bson::<User>(bson::Bson::Document(doc)).context("error-unknown")?;
            let hashed_password = blake2_str(info.password.as_bytes());
            return if user.password == hashed_password {
                let login_token = blake2_str(user.mail.as_bytes());

                let new_record = doc! {
                    "login_token": login_token.clone(),
                    "login_time": Utc::now(),
                };
                self.login_records.insert_one(new_record, None).await.context("error-net")?;
                Ok((login_token, user.mail, user.name))
            } else {
                bail!("error-wrong-password")
            };
        }
        bail!("error-no-user")
    }

    pub async fn register(&self, info: RegisterRequest) -> anyhow::Result<()> {
        let filter = doc! {
            "mail": &info.mail,
        };
        if let Some(_) = self.users.find_one(filter, None).await.context("error-net")? {
            bail!("error-dup-email");
        }

        let filter = doc! {
            "name": &info.name,
        };
        if let Some(_) = self.users.find_one(filter, None).await.context("error-net")? {
            bail!("error-dup-username");
        }

        let hashed_password = blake2_str(info.password.as_bytes());
        let user = User {
            _id: None,
            mail: info.mail,
            name: info.name,
            password: hashed_password,
            devices: vec![],
        };
        let serialized_user = bson::to_bson(&user).context("error-unknown")?;
        let doc = serialized_user.as_document().context("error-unknown")?;
        self.users.insert_one(doc.to_owned(), None).await.context("error-net")?;
        Ok(())
    }

    pub async fn logout(&self, login_token: &str) -> anyhow::Result<()> {
        let filter = doc! {
            "login_token": login_token,
        };
        if let Some(_) = self
            .login_records
            .find_one(filter.clone(), None)
            .await
            .context("error-net")?
        {
            self.login_records.delete_many(filter, None).await.context("error-net")?;
        }
        Ok(())
    }

    pub async fn insert_message(&self, msg: Message) -> anyhow::Result<()> {
        let serialized_msg = bson::to_bson(&msg).context("error-unknown")?;
        let doc = serialized_msg.as_document().context("error-unknown")?;
        self.messages.insert_one(doc.to_owned(), None).await.context("error-net")?;
        Ok(())
    }

    pub async fn create_device(&self, info: CreateDeviceRequest) -> anyhow::Result<()> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired");
        }

        let filter = doc! {
            "mail": info.mail.clone(),
        };
        if let None = self.users.find_one(filter, None).await.context("error-net")? {
            bail!("error-no-user");
        }

        let filter = doc! {
            "id": info.id.clone()
        };
        if let None = self.devices.find_one(filter, None).await.context("error-net")? {
            let dev = Device {
                _id: None,
                id: info.id.clone(),
                name: info.id.clone(),
                info: "".to_string(),
            };
            let serialized_dev = bson::to_bson(&dev).context("error-unknown")?;
            let doc = serialized_dev.as_document().context("error-unknown")?;
            self.devices.insert_one(doc.to_owned(), None).await.context("error-net")?;
        }

        let query = doc! {
            "mail": info.mail
        };
        let update = doc! {
            "$push": {
                "devices": info.id.clone(),
            }
        };
        self.users.update_one(query, update, None).await.context("error-net")?;
        Ok(())
    }

    pub async fn remove_device(&self, info: RemoveDeviceRequest) -> anyhow::Result<()> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired");
        }

        let filter = doc! {
            "mail": info.mail.clone(),
        };
        if let None = self.users.find_one(filter, None).await.context("error-net")? {
            bail!("error-no-user");
        }

        let filter = doc! {
            "mail": info.mail.clone(),
            "devices": {
                "$elemMatch": {
                    "$eq": info.id.clone()
                }
            }
        };
        if let None = self.users.find_one(filter, None).await.context("error-net")? {
            bail!("error-no-device");
        }

        let query = doc! {
            "mail": info.mail
        };
        let update = doc! {
            "$pull": {
                "devices": info.id.clone(),
            }
        };
        self.users.update_one(query, update, None).await.context("error-net")?;
        Ok(())
    }

    pub async fn modify_device(&self, info: ModifyDeviceRequest) -> anyhow::Result<()> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired");
        }

        let filter = doc! {
            "id": info.id.clone(),
        };
        let device = self.devices.find_one(filter, None).await.context("error-net")?;
        if device.is_none() {
            bail!("error-no-device");
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
        self.devices.update_one(query, update, None).await.context("error-net")?;
        Ok(())
    }

    pub async fn fetch_device(
        &self,
        info: FetchDeviceRequest,
    ) -> anyhow::Result<(String, String, String)> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired".to_string());
        }

        let filter = doc! {
            "id": info.id
        };
        let device = self.devices.find_one(filter, None).await.context("error-net")?;
        if device.is_none() {
            bail!("error-no-device");
        }
        let device: Device = bson::from_bson(bson::Bson::Document(device.context("error-unknown")?)).context("error-unknown")?;
        Ok((device.id, device.name, device.info))
    }

    pub async fn fetch_device_profile(
        &self,
        info: FetchDeviceProfileRequest,
    ) -> anyhow::Result<DeviceInfo> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired");
        }

        let filter = doc! {
            "id": info.id.clone()
        };
        if let Some(dev) = self.devices.find_one(filter, None).await.context("error-net")? {
            let dev: Device = bson::from_bson(bson::Bson::Document(dev)).context("error-unknown")?;

            let count_filter = doc! {
                "id": dev.id.clone(),
            };
            let message_count = self
                .messages
                .count_documents(count_filter, None)
                .await
                .context("error-net")? as u32;
            let count_filter = doc! {
                "id": dev.id.clone(),
                "alert": true,
            };
            let alert_message_count = self
                .messages
                .count_documents(count_filter, None)
                .await
                .context("error-net")? as u32;

            Ok(DeviceInfo {
                id: dev.id,
                name: dev.name,
                message_count,
                alert_message_count,
            })
        } else {
            bail!("error-no-device")
        }
    }

    pub async fn fetch_device_list(
        &self,
        info: FetchDeviceListRequest,
    ) -> anyhow::Result<Vec<DeviceInfo>> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired".to_string());
        }

        let filter = doc! {
            "mail": info.mail.clone(),
        };
        let user = self.users.find_one(filter, None).await.context("error-net")?;
        if user.is_none() {
            bail!("error-no-user");
        }
        let user: User = bson::from_bson(bson::Bson::Document(user.context("error-unknown")?)).context("error-unknown")?;

        let mut devices = Vec::with_capacity(user.devices.len());
        for id in &user.devices {
            let filter = doc! {
                "id": id.clone()
            };
            if let Some(dev) = self.devices.find_one(filter, None).await.context("error-net")? {
                let dev: Device = bson::from_bson(bson::Bson::Document(dev)).context("error-unknown")?;

                let count_filter = doc! {
                    "id": dev.id.clone(),
                };
                let message_count = self
                    .messages
                    .count_documents(count_filter, None)
                    .await
                    .context("error-net")? as u32;
                let count_filter = doc! {
                    "id": dev.id.clone(),
                    "alert": true,
                };
                let alert_message_count = self
                    .messages
                    .count_documents(count_filter, None)
                    .await
                    .context("error-net")? as u32;

                let dev = DeviceInfo {
                    id: dev.id,
                    name: dev.name,
                    message_count,
                    alert_message_count,
                };
                devices.push(dev);
            } else {
                bail!("error-no-device");
            }
        }

        Ok(devices)
    }

    pub async fn fetch_message_list(
        &self,
        info: FetchMessageListRequest,
    ) -> anyhow::Result<Vec<MessageInfo>> {
        if !self.check_login(&info.login_token).await? {
            bail!("Login has expired");
        }

        let filter = doc! {
            "id": info.id.clone(),
            "timestamp": {
                "$gte": info.start_timestamp,
                "$lte": info.end_timestamp,
            }
        };
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();
        let mut cursor = self
            .messages
            .find(filter, find_options)
            .await
            .context("error-net")?
            .skip(info.first_index);
        let mut messages = vec![];
        while let Some(msg) = cursor.next().await {
            let msg: Message = bson::from_bson(bson::Bson::Document(msg.context("error-unknown")?)).context("error-unknown")?;
            let msg = MessageInfo {
                id: msg.id,
                info: msg.info,
                value: msg.value as u32,
                alert: msg.alert,
                lng: msg.lng,
                lat: msg.lat,
                timestamp: msg.timestamp,
            };
            messages.push(msg);
            if messages.len() == info.limit {
                break;
            }
        }

        Ok(messages)
    }

    const MAX_LOGIN_TIME_SECS: i64 = 3600;

    pub async fn check_login(&self, login_token: &str) -> anyhow::Result<bool> {
        let filter = doc! {
            "login_token": login_token
        };
        let find_options = FindOneOptions::builder()
            .sort(doc! { "login_time": -1 })
            .build();
        if let Some(record) = self
            .login_records
            .find_one(filter.clone(), find_options)
            .await
            .context("error-net")?
        {
            let login_time = record.get_datetime("login_time").context("error-unknown")?;
            let now_time = Utc::now();
            let diff = now_time
                .naive_utc()
                .signed_duration_since(login_time.naive_utc());
            if diff.num_seconds() > Self::MAX_LOGIN_TIME_SECS {
                self.login_records.delete_one(filter, None).await.context("error-net")?;
            } else {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Message {
    pub fn new(
        id: String,
        info: String,
        value: i32,
        alert: bool,
        lng: f64,
        lat: f64,
        timestamp: i64,
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

fn blake2_str(input: &[u8]) -> String {
    use blake2::{Blake2b, Digest};
    format!("{:x}", Blake2b::digest(input))
}
