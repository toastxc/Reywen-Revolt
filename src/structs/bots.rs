use serde::{Deserialize, Serialize};

/// Utility function to check if a boolean value is false
fn if_false(t: &bool) -> bool {
    !t
}

/// Bot flag enum
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum BotFlags {
    Verified = 1,
    Official = 2,
}

/// Representation of a bot on Revolt
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum FieldsBot {
    Token,
    InteractionsURL,
}
