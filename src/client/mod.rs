use reywen_http::{results::DeltaError, Delta};

use crate::websocket::WebSocket;

#[derive(Debug, Clone, Default)]
pub struct Client {
    pub http: Delta,
    pub websocket: WebSocket,
    pub token: String,
}

impl Client {
    pub fn from_token(token: &str, is_bot: bool) -> Result<Self, DeltaError> {
        let http = Delta::new()
            .set_url("https://api.revolt.chat/")
            .add_header(
                match is_bot {
                    true => "x-bot-token",
                    false => "x-session-token",
                },
                token,
            )?
            .set_timeout(10);

        Ok(Self {
            http,
            websocket: WebSocket::from_token(token),
            token: String::from(token),
        })
    }
    pub fn new() -> Self {
        Default::default()
    }
}

pub mod methods;
