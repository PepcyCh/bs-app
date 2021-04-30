use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    addr_ip: String,
    addr_port: String,
    db_username: String,
    db_password: String,
    db_url: String,
}

lazy_static! {
    static ref ADDR_IP_RE: Regex = Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    static ref ADDR_PORT_RE: Regex = Regex::new(r"^(6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|6[0-4][0-9]{3}|[1-5][0-9]{4}|[1-9][0-9]{1,3})$").unwrap();
}

impl ServerConfig {
    pub fn validate(&self) -> bool {
        ADDR_IP_RE.is_match(&self.addr_ip) && ADDR_PORT_RE.is_match(&self.addr_port)
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.addr_ip, self.addr_port)
    }

    pub fn db_url(&self) -> String {
        format!(
            "mongodb+srv://{}:{}@{}?retryWrites=true&w=majority",
            self.db_username, self.db_password, self.db_url
        )
    }
}
