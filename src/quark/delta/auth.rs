use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Auth {
    pub token: String,
    pub bot_id: String,
    pub sudoers: Vec<String>,
    #[serde(rename = "api_domain")]
    pub domain: String,
}
