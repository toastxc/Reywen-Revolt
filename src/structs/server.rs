use std::collections::HashMap;

use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::attachment::File;

fn if_false(t: &bool) -> bool {
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

impl Role {
    pub fn new() -> Self {
        Role::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    pub fn colour(mut self, colour: &str) -> Self {
        self.colour = Some(String::from(colour));
        self
    }
    pub fn hoist(mut self, hoist: bool) -> Self {
        self.hoist = hoist;
        self
    }
    pub fn rank(mut self, rank: i64) -> Self {
        self.rank = rank;
        self
    }
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

impl Category {
    pub fn id(mut self, id: &str) -> Self {
        self.id = String::from(id);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn channels(mut self, channels: Vec<String>) -> Self {
        self.channels = channels;
        self
    }

    pub fn channel_add(mut self, channel: &str) -> Self {
        match self.channels.is_empty() {
            true => self.channels = vec![String::from(channel)],
            false => self.channels.push(String::from(channel)),
        };
        self
    }
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

// ########################## SERVER MEMBERS ##########################
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemberCompositeKey {
    /// Server Id
    pub server: String,
    /// User Id
    pub user: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,

    /// Time at which this user joined the server
    pub joined_at: Timestamp,

    /// Member's nickname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Avatar attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,

    /// Member's roles
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub roles: Vec<String>,
    /// Timestamp this member is timed out until
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Timestamp>,
}
