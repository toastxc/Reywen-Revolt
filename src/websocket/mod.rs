pub mod data;
pub mod process;
pub mod result;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use self::data::WebSocketSend;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WebSocket {
    pub token: String,
    pub format: String,
    #[serde(rename = "websocket_domain")]
    pub domain: String,
}

impl WebSocket {
    pub fn from_token(token: &str) -> Self {
        WebSocket {
            token: String::from(token),
            format: String::from("json"),
            domain: String::from("ws.revolt.chat"),
        }
    }

    pub fn ws_send(input: WebSocketSend) -> Message {
        Message::from(serde_json::to_string(&input).unwrap())
    }
}
impl From<WebSocketSend> for Message {
    fn from(value: WebSocketSend) -> Self {
        Message::from(serde_json::to_string(&value).unwrap())
    }
}
