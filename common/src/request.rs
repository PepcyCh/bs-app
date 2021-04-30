use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct LoginRequest {
    pub mail: String,
    pub password: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub mail: String,
    pub name: String,
    pub password: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct CreateDeviceRequest {
    pub mail: String,
    pub id: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct RemoveDeviceRequest {
    pub mail: String,
    pub id: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ModifyDeviceNameRequest {
    pub id: String,
    pub name: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ModifyDeviceInfoRequest {
    pub id: String,
    pub info: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchDevicesRequest {
    pub mail: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchMessagesRequest {
    pub id: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
}

impl Default for FetchMessagesRequest {
    fn default() -> Self {
        Self {
            id: String::default(),
            start_timestamp: 0,
            end_timestamp: std::u64::MAX,
        }
    }
}
