use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    client::Web,
    structs::{
        attachment::File,
        bots::{Bot, FieldsBot},
        user::User,
    },
};
/// # Bot Details

#[derive(Validate, Deserialize, Serialize)]
pub struct DataCreateBot {
    /// Bot username
    #[validate(length(min = 2, max = 32))]
    name: String,
}

#[allow(dead_code)]
pub async fn create(domain: &str, token: &str, data: DataCreateBot) -> Option<Bot> {
    match reqwest::Client::new()
        .post(format!("https://{domain}/bots/create"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "create_bot");
            None
        }
        Ok(a) => match serde_json::from_str::<Bot>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

/// # Public Bot
#[derive(Serialize, Deserialize)]
pub struct PublicBot {
    /// Bot Id
    #[serde(rename = "_id")]
    id: String,
    /// Bot Username
    username: String,
    /// Profile Avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<File>,
    /// Profile Description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[allow(dead_code)]
pub async fn fetch_public(domain: &str, token: &str, bot: &str) -> Option<PublicBot> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/bots/{bot}/invite"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "fetch_public_bot");
            None
        }
        Ok(a) => match serde_json::from_str::<PublicBot>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
/// # Invite Destination
#[derive(Deserialize, Serialize)]
pub enum InviteBotDestination {
    /// Invite to a server
    Server {
        /// Server Id
        server: String,
    },
    /// Invite to a group
    Group {
        /// Group Id
        group: String,
    },
}

#[allow(dead_code)]
pub async fn invite(domain: &str, token: &str, bot: &str, data: InviteBotDestination) {
    if let Err(e) = reqwest::Client::new()
        .post(format!("https://{domain}/bots/{bot}/invite"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
    {
        Web::error(e, "invite_bot");
    }
}
/// # Bot Response
#[derive(Serialize, Deserialize)]
pub struct BotResponse {
    /// Bot object
    bot: Bot,
    /// User object
    user: User,
}

#[allow(dead_code)]
pub async fn fetch(domain: &str, token: &str, bot: &str) -> Option<BotResponse> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/bots/{bot}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "fetch_public_bot");
            None
        }
        Ok(a) => match serde_json::from_str::<BotResponse>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn delete(domain: &str, token: &str, bot: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/bots/{bot}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Web::error(e, "delete_bot");
    }
}
/// # Bot Details
#[derive(Validate, Serialize, Deserialize)]
pub struct DataEditBot {
    /// Bot username
    #[validate(length(min = 2, max = 32))]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Whether the bot can be added by anyone
    public: Option<bool>,
    /// Whether analytics should be gathered for this bot
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    analytics: Option<bool>,
    /// Interactions URL
    #[validate(length(min = 1, max = 2048))]
    interactions_url: Option<String>,
    /// Fields to remove from bot object
    #[validate(length(min = 1))]
    remove: Option<Vec<FieldsBot>>,
}
#[allow(dead_code)]
pub async fn edit(domain: &str, token: &str, bot: &str) -> Option<Bot> {
    match reqwest::Client::new()
        .patch(format!("https://{domain}/bots/{bot}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "create_bot");
            None
        }
        Ok(a) => match serde_json::from_str::<Bot>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

/// # Owned Bots Response
///
/// Both lists are sorted by their IDs.
#[derive(Serialize, Deserialize)]
pub struct OwnedBotsResponse {
    /// Bot objects
    bots: Vec<Bot>,
    /// User objects
    users: Vec<User>,
}

#[allow(dead_code)]
pub async fn owned(domain: &str, token: &str) -> Option<OwnedBotsResponse> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/bots/@me"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "create_bot");
            None
        }
        Ok(a) => match serde_json::from_str::<OwnedBotsResponse>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}