use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginInfo {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub err: String,
    pub mail: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub mail: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub err: String,
}
