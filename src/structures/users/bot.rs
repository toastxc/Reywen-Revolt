use reywen_http::utils::if_false;
use serde::{Deserialize, Serialize};

use crate::structures::media::attachment::File;

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
