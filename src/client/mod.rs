use crate::{
    reywen_http::{results::DeltaError, Delta},
    websocket::WebSocket,
};
use std::fmt::Debug;
pub mod methods;
#[derive(Debug, Clone)]
pub struct Client {
    pub http: Delta,
    pub websocket: WebSocket,
    pub token: Option<String>,
    pub self_id: Option<String>,
    pub is_bot: bool,
}

impl Client {
    pub fn from_token(
        token: impl Into<String>,
        user_id: impl Into<String>,
        is_bot: bool,
    ) -> Result<Self> {
        Self::default()
            .set_token(token)
            .set_id(user_id)
            .set_is_bot(is_bot)
            .make()
    }

    pub fn set_engine(&mut self, engine: Delta) -> Self {
        self.http = engine;
        self.clone()
    }
    pub fn set_websocket(&mut self, websocket: WebSocket) -> Self {
        self.websocket = websocket;
        self.clone()
    }
    pub fn set_token(&mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self.clone()
    }
    pub fn set_id(&mut self, id: impl Into<String>) -> Self {
        self.self_id = Some(id.into());
        self.clone()
    }

    pub fn set_is_bot(&mut self, is_bot: bool) -> Self {
        self.is_bot = is_bot;
        self.clone()
    }
    pub fn make(&mut self) -> Result<Self> {
        self.http.add_header(
            if self.is_bot {
                "x-bot-token"
            } else {
                "x-session-token"
            },
            self.token.clone().unwrap_or("NOTVALIDTOKEN".to_string()),
        )?;

        self.websocket.token = self.token.clone();
        Ok(self.clone())
    }
    // pub fn from_token(token: &str, is_bot: bool) -> Result<Self> {
    //     Self::from_token_url(token, is_bot, None)
    // }
    // pub fn from_token_url(token: &str, is_bot: bool, url: Option<&str>) -> Result<Self> {
    //     // deriving defaults for types
    //     let mut client = Self::default();
    //     client.websocket.token = Some(String::from(token));
    //     client.token = Some(String::from(token));
    //
    //     client.http.add_header(
    //         if is_bot {
    //             "x-bot-token"
    //         } else {
    //             "x-session-token"
    //         },
    //         token,
    //     )?;
    //     if let Some(custom_url) = url {
    //         client.http.set_url(custom_url);
    //     };
    //
    //     Ok(client)
    // }
    // pub fn new() -> Self {
    //     Default::default()
    // }
}
impl Default for Client {
    fn default() -> Self {
        Self {
            http: Delta::new().set_url("https://app.revolt.chat/api"),
            websocket: Default::default(),
            token: None,
            self_id: None,
            is_bot: false,
        }
    }
}

pub type Result<T> = core::result::Result<T, DeltaError>;
