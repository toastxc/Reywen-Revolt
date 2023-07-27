use reywen_http::{results::DeltaError, Delta};

use crate::websocket::WebSocket;

#[derive(Debug, Clone)]
pub struct Client {
    pub http: Delta,
    pub websocket: WebSocket,
    pub token: String,
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
        let mut client = Self::default();
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
        let http = Delta::new()
            .set_url("https://api.revolt.chat")
            .set_timeout(10);

        Self {
            http,
            websocket: Default::default(),
            token: String::from("INSERT_TOKEN"),
        }
    }
}

pub mod methods;
