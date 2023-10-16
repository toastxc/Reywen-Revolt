use crate::websocket::WebSocket;
use reywen_http::{results::DeltaError, Delta};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Client {
    pub http: Delta,
    pub websocket: WebSocket,
    pub token: Option<String>,
    pub autumn_uri: String,
}

impl Client {
    pub fn from_token(token: &str, is_bot: bool) -> Result<Self, DeltaError> {
        Self::from_token_url(token, is_bot, None)
    }

    pub fn from_token_url(
        token: &str,
        is_bot: bool,
        url: Option<&str>,
    ) -> Result<Self, DeltaError> {
        // derinvg defaults for types
        let mut client = Self::default();
        client.websocket.token = Some(String::from(token));
        client.token = Some(String::from(token));

        client.http.add_header(
            if is_bot {
                "x-bot-token"
            } else {
                "x-session-token"
            },
            token,
        )?;
        if let Some(custom_url) = url {
            client.http.set_url(custom_url);
        };

        Ok(client)
    }
    pub fn new() -> Self {
        Default::default()
    }
}
impl Default for Client {
    fn default() -> Self {
        Self {
            http: Delta::new()
                .set_url("https://api.revolt.chat")
                .set_timeout(10),

            websocket: WebSocket::default(),
            token: None,
            autumn_uri: String::from("https://autumn.revolt.chat"),
        }
    }
}
pub mod methods;
