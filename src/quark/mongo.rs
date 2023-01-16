use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RMongo {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub database: String
}