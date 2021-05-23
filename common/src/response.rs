use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct SimpleResponse {
    pub success: bool,
    pub err: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub err: String,
    pub login_token: String,
    pub mail: String,
    pub name: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchDeviceResponse {
    pub success: bool,
    pub err: String,
    pub id: String,
    pub name: String,
    pub info: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub message_count: u32,
    pub alert_message_count: u32,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchDeviceListResponse {
    pub success: bool,
    pub err: String,
    pub devices: Vec<DeviceInfo>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct MessageInfo {
    pub id: String,
    pub info: String,
    pub value: u32,
    pub alert: bool,
    pub lng: f64,
    pub lat: f64,
    pub timestamp: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct FetchMessageListResponse {
    pub success: bool,
    pub err: String,
    pub messages: Vec<MessageInfo>,
}

pub trait ErrorResponse {
    fn err<S: ToString>(info: S) -> Self;
}

macro_rules! error_response_impl {
    ( $( $type:ty ),+ $(,)? ) => {
        $(
            impl ErrorResponse for $type {
                fn err<S: ToString>(info: S) -> Self {
                    Self {
                        success: false,
                        err: info.to_string(),
                        ..Default::default()
                    }
                }
            }
        )*
    };
}

error_response_impl! {
    SimpleResponse,
    LoginResponse,
    FetchDeviceResponse,
    FetchDeviceListResponse,
    FetchMessageListResponse,
}
