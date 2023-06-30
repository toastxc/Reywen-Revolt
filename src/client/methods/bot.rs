use reywen_http::results::{result, DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    structures::users::{
        bot::{Bot, FieldsBot, PublicBot},
        user::User,
    },
};

impl Client {
    pub async fn bot_create(&self, data: &DataCreateBot) -> Result<Bot, DeltaError> {
        result(
            self.http
                .post("/bots/create", Some(&serde_json::to_string(&data).unwrap()))
                .await,
        )
        .await
    }
    pub async fn bot_delete(&self, bot_id: &str) -> Result<(), DeltaError> {
        result(self.http.delete(&format!("/bots/{bot_id}"), None).await).await
    }

    pub async fn bot_edit(&self, bot_id: &str, data: &DataEditBot) -> Result<Bot, DeltaError> {
        result(
            self.http
                .patch(
                    &format!("/bots/{bot_id}"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn bot_fetch(&self, bot_id: &str) -> Result<BotResponse, DeltaError> {
        result(self.http.get(&format!("/bots/{bot_id}")).await).await
    }
    pub async fn bot_fetch_owned(&self) -> Result<OwnedBotsResponse, DeltaError> {
        result(self.http.get("/bots/@me").await).await
    }
    pub async fn bot_fetch_public(&self, bot_id: &str) -> Result<PublicBot, DeltaError> {
        result(self.http.get(&format!("/bots/{bot_id}/invite")).await).await
    }

    pub async fn bot_invite(&self, bot_id: &str, server_id: &str) -> Result<(), DeltaError> {
        let data = DataBotInvite {
            server: server_id.to_string(),
        };
        result(
            self.http
                .post(
                    &format!("/bots/{bot_id}/invite"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataBotInvite {
    pub server: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OwnedBotsResponse {
    pub bots: Vec<Bot>,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotResponse {
    pub bot: Bot,
    pub user: User,
}

/// # Bot Details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCreateBot {
    /// Bot username
    pub name: String,
}

impl DataCreateBot {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

/// # Bot Details
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataEditBot {
    /// Bot username
    pub name: Option<String>,
    /// Whether the bot can be added by anyone
    pub public: Option<bool>,
    /// Whether analytics should be gathered for this bot
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    pub analytics: Option<bool>,
    /// Interactions URL
    pub interactions_url: Option<String>,
    /// Fields to remove from bot object
    pub remove: Option<Vec<FieldsBot>>,
}

impl DataEditBot {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_public(&mut self, public: bool) -> Self {
        self.public = Some(public);
        self.to_owned()
    }
    pub fn set_analytics(&mut self, analytics: bool) -> Self {
        self.analytics = Some(analytics);
        self.to_owned()
    }
    pub fn set_interactions_url(&mut self, interactions_url: &str) -> Self {
        self.interactions_url = Some(String::from(interactions_url));
        self.to_owned()
    }

    pub fn add_remove(&mut self, field: FieldsBot) -> Self {
        match self.remove.clone() {
            Some(mut old) => old.push(field),
            None => self.remove = Some(vec![field]),
        }
        self.to_owned()
    }
    pub fn set_remove(&mut self, fields: Vec<FieldsBot>) -> Self {
        self.remove = Some(fields);
        self.to_owned()
    }
}
