use crate::impl_to_vec;
use crate::reywen_http::utils::if_false;
use serde::{Deserialize, Serialize};

use crate::structures::media::attachment::File;
use crate::structures::users::User;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PublicBot {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub avatar: Option<File>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Bot {
    /// Bot Id
    ///
    /// This equals the associated bot user's id.
    #[serde(rename = "_id")]
    pub id: String,
    /// User Id of the bot owner
    pub owner: String,
    /// Token used to authenticate requests for this bot
    pub token: String,
    /// Whether the bot is public
    /// (may be invited by anyone)
    pub public: bool,
    /// Whether to enable analytics
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    /// Whether this bot should be publicly discoverable
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
    /// Reserved; URL for handling interactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<String>,
    /// URL for terms of service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    /// URL for privacy policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// Enum of bot flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
}

/// Optional fields on bot object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldsBot {
    Token,
    InteractionsURL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataBotInvite {
    Server { server: String },
    Group { group: String },
}
impl_to_vec!(DataBotInvite);

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

impl_to_vec!(DataCreateBot);

impl DataCreateBot {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
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
impl_to_vec!(DataEditBot);
impl DataEditBot {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self.clone()
    }
    pub fn set_public(&mut self, public: bool) -> Self {
        self.public = Some(public);
        self.clone()
    }
    pub fn set_analytics(&mut self, analytics: bool) -> Self {
        self.analytics = Some(analytics);
        self.clone()
    }
    pub fn set_interactions_url(&mut self, interactions_url: impl Into<String>) -> Self {
        self.interactions_url = Some(interactions_url.into());
        self.clone()
    }

    pub fn add_remove(&mut self, field: impl Into<FieldsBot>) -> Self {
        match self.remove.clone() {
            Some(mut old) => old.push(field.into()),
            None => self.remove = Some(vec![field.into()]),
        }
        self.clone()
    }
    pub fn set_remove(&mut self, fields: impl Into<Vec<FieldsBot>>) -> Self {
        self.remove = Some(fields.into());
        self.clone()
    }
}
