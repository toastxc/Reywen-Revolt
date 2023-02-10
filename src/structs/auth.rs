use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Auth {
    pub token: String,
    pub bot_id: String,
    pub sudoers: Vec<String>,
    #[serde(rename = "api_domain")]
    pub domain: String,
    pub header: String,
}

impl Auth {
    pub fn from_token(token: &str, is_bot: bool) -> Self {
        let is_bot = match is_bot {
            true => "x-bot-token",
            false => "x-session-token",
        }
        .to_string();

        Auth {
            token: String::from(token),
            domain: (String::from("api.revolt.chat")),
            header: is_bot,
            ..Default::default()
        }
    }
}
