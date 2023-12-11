use crate::{impl_to_vec, reywen_http::utils::if_false, structures::media::attachment::File};
use std::collections::HashMap;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use crate::client::methods::opt_vec_add;
use crate::structures::channels::Channel;
use crate::structures::server::ban::ServerBan;
use crate::structures::users::User;


pub mod ban;
pub mod member;
/// Representation of a server role
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Role {
    /// Role name
    pub name: String,
    /// Permissions available to this role
    //pub permissions: OverrideField,
    /// Colour used for this role
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

/// Channel category
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    /// Unique ID for this category
    pub id: String,
    /// Title for this
    pub title: String,
    /// Channels in this category
    pub channels: Vec<String>,
}

/// System message channel assignments
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

/// Server flag enum
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(i32)]
pub enum ServerFlags {
    Verified = 1,
    Official = 2,
}

/// Representation of a server on Revolt
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

    /// Icon attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<File>,
    /// Banner attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<File>,

    /// Bitfield of server flags
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

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsServer {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}

/// Optional fields on server object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsRole {
    Colour,
}




#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelCreate {
    #[serde(rename = "type")]
    pub r#type: Option<ChannelType>,
    pub name: String,
    pub description: Option<String>,
    pub nsfw: Option<bool>,
}
impl_to_vec!(DataChannelCreate);
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
}

impl DataChannelCreate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn set_type(&mut self, r#type: impl Into<ChannelType>) -> Self {
        self.r#type = Some(r#type.into());
        self.clone()
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.clone()
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self.clone()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BannedUser {
    /// Id of the banned user
    #[serde(rename = "_id")]
    pub id: String,
    /// Username of the banned user
    pub username: String,
    /// Avatar of the banned user
    pub avatar: Option<File>,
}

/// # Ban List Result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBanList {
    /// Users objects
    users: Vec<BannedUser>,
    /// Ban objects
    bans: Vec<ServerBan>,
}

impl From<User> for BannedUser {
    fn from(user: User) -> Self {
        BannedUser {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataCreateServer {
    /// Server name
    name: String,
    /// Server description
    description: Option<String>,
    /// Whether this server is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}
impl_to_vec!(DataCreateServer);
impl DataCreateServer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.clone()
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self.clone()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.clone()
    }
}

/// # Create Server Response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerResponse {
    /// Server object
    server: Server,
    /// Default channels
    channels: Vec<Channel>,
}

/// # Server Data
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataEditServer {
    /// Server name
    pub name: Option<String>,
    /// Server description
    pub description: Option<String>,
    /// Attachment Id for icon
    pub icon: Option<String>,
    /// Attachment Id for banner
    pub banner: Option<String>,
    /// Category structure for server
    pub categories: Option<Vec<Category>>,
    /// System message configuration
    pub system_messages: Option<SystemMessageChannels>,

    /// Bitfield of server flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    // Whether this server is age-restricted
    // nsfw: Option<bool>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    pub discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    pub analytics: Option<bool>,

    /// Fields to remove from server object
    pub remove: Option<Vec<FieldsServer>>,
}
impl_to_vec!(DataEditServer);
impl DataEditServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self.clone()
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.name = Some(description.into());
        self.clone()
    }
    pub fn set_icon(&mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.clone()
    }
    pub fn set_banner(&mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self.clone()
    }
    pub fn add_category(&mut self, category: impl Into<Category>) -> Self {
        opt_vec_add(&mut self.categories, category.into());
        self.clone()
    }
    pub fn set_categories(&mut self, categories: impl Into<Vec<Category>>) -> Self {
        self.categories = Some(categories.into());
        self.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBanReason {
    pub reason: Option<String>,
}
impl_to_vec!(DataBanReason);

impl From<Option<&str>> for DataBanReason {
    fn from(value: Option<&str>) -> Self {
        Self {
            reason: value.map(|reason| reason.to_string()),
        }
    }
}

impl DataBanReason {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: Some(reason.into()),
        }
    }
    pub fn none() -> Self {
        Self { reason: None }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBan {
    pub _id: BanId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BanId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    pub user: Option<String>,
}
