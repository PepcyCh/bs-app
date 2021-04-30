use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginInfo {
    pub mail: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub err: String,
    pub mail: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct RegisterInfo {
    pub mail: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub err: String,
}
