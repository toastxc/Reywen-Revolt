use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RWebsocket {
    pub token: String,
    pub format: String,
    #[serde(rename = "websocket_domain")]
    pub domain: String,
}
