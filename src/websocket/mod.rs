pub mod data;
pub mod process;
pub mod result;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

use self::data::WebSocketSend;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebSocket {
    pub token: Option<String>,
    pub format: Option<String>,
    #[serde(rename = "websocket_domain")]
    pub domain: Option<String>,
    pub version: Option<u16>,
}

#[derive(Serialize)]
pub struct PartialWSConf {
    pub version: Option<u16>,
    pub format: Option<String>,
    pub token: Option<String>,
}
impl From<WebSocket> for PartialWSConf {
    fn from(value: WebSocket) -> Self {
        Self {
            version: value.version,
            format: value.format,
            token: value.token,
        }
    }
}
impl WebSocket {
    pub fn from_token(token: &str) -> Self {
        WebSocket {
            token: Some(String::from(token)),
            ..Default::default()
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

impl Default for WebSocket {
    fn default() -> Self {
        Self {
            token: None,
            format: Some(String::from("json")),
            domain: Some(String::from("ws.revolt.chat")),
            version: Some(1),
        }
    }
}
impl Default for PartialWSConf {
    fn default() -> Self {
        WebSocket::default().into()
    }
}
