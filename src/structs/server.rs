use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn if_false(t: &bool) -> bool {
    !t
}

/// Representation of a server role
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Role {
    /// Role name
    pub name: String,
    ///
    /// This can be any valid CSS colour
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
    /// Whether this role should be shown separately on the member sidebar
    #[serde(skip_serializing_if = "if_false", default)]
    pub hoist: bool,
    /// Ranking of this role
    #[serde(default)]
    pub rank: i64,
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    /// Unique ID for this category
    #[validate(length(min = 1, max = 32))]
    pub id: String,
    /// Title for this category
    #[validate(length(min = 1, max = 32))]
    pub title: String,
    /// Channels in this category
    pub channels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemMessageChannels {
    /// ID of channel to send user join messages in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<String>,
    /// ID of channel to send user left messages in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_left: Option<String>,
    /// ID of channel to send user kicked messages in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<String>,
    /// ID of channel to send user banned messages in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum ServerFlags {
    Verified = 1,
    Official = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Server {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// User id of the owner
    pub owner: String,

    /// Name of the server
    pub name: String,
    /// Description for the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Channels within this server
    // ! FIXME: this may be redundant
    pub channels: Vec<String>,
    /// Categories for this server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    /// Configuration for sending system event messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<SystemMessageChannels>,

    /// Roles for this server
    #[serde(
        default = "HashMap::<String, Role>::new",
        skip_serializing_if = "HashMap::<String, Role>::is_empty"
    )]
    pub roles: HashMap<String, Role>,
    /// Default set of server and channel permissions
    pub default_permissions: i64,

    /// Enum of server flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    /// Whether this server is flagged as not safe for work
    #[serde(skip_serializing_if = "if_false", default)]
    pub nsfw: bool,
    /// Whether to enable analytics
    #[serde(skip_serializing_if = "if_false", default)]
    pub analytics: bool,
    /// Whether this server should be publicly discoverable
    #[serde(skip_serializing_if = "if_false", default)]
    pub discoverable: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsRole {
    Colour,
}