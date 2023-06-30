pub mod data;
pub mod process;
pub mod result;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Websocket {
    pub token: String,
    pub format: String,
    #[serde(rename = "websocket_domain")]
    pub domain: String,
}

impl Websocket {
    pub fn from_token(token: &str) -> Self {
        Websocket {
            token: String::from(token),
            format: String::from("json"),
            domain: String::from("ws.revolt.chat"),
        }
    }

    pub async fn ws_send(input: &str) -> Message {
        Message::from(input)
    }
}
