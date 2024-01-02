use crate::impl_to_vec;
use crate::reywen_http::utils::if_false;
use crate::structures::media::attachment::File;
use serde::{Deserialize, Serialize};

pub mod group;
pub mod invite;
pub mod message;

/// Representation of a channel on Revolt
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    /// Personal "Saved Notes" channel which allows users to save messages
    SavedMessages {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the user this channel belongs to
        user: String,
    },
    /// Direct message channel between two users
    DirectMessage {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,

        /// Whether this direct message channel is currently open on both sides
        active: bool,
        /// 2-tuple of user ids participating in direct message
        recipients: Vec<String>,
        /// Id of the last message sent in this channel
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,
    },
    /// Group channel between 1 or more participants
    Group {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,

        /// Display name of the channel
        name: String,
        /// User id of the owner of the group
        owner: String,
        /// Channel description
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Array of user ids participating in channel
        recipients: Vec<String>,

        /// Custom icon attachment
        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,
        /// Id of the last message sent in this channel
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,

        /// Permissions assigned to members of this group
        /// (does not apply to the owner of the group)
        #[serde(skip_serializing_if = "Option::is_none")]
        permissions: Option<i64>,

        /// Whether this group is marked as not safe for work
        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
    /// Text channel belonging to a server
    TextChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the server this channel belongs to
        server: String,

        /// Display name of the channel
        name: String,
        /// Channel description
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        /// Custom icon attachment
        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,
        /// Id of the last message sent in this channel
        #[serde(skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,

        /// Whether this channel is marked as not safe for work
        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
    /// Voice channel belonging to a server
    VoiceChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the server this channel belongs to
        server: String,

        /// Display name of the channel
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Channel description
        description: Option<String>,
        /// Custom icon attachment
        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<File>,

        /// Whether this channel is marked as not safe for work
        #[serde(skip_serializing_if = "if_false", default)]
        nsfw: bool,
    },
}

impl Channel {
    pub fn id(&self) -> String {
        match self {
            Channel::DirectMessage { id, .. }
            | Channel::Group { id, .. }
            | Channel::SavedMessages { id, .. }
            | Channel::TextChannel { id, .. }
            | Channel::VoiceChannel { id, .. } => id.clone(),
        }
    }

    pub fn server_id(&self) -> Option<String> {
        match self {
            Channel::TextChannel { server, .. } | Channel::VoiceChannel { server, .. } => {
                Some(server.clone())
            }
            _ => None,
        }
    }
    pub fn name(&self) -> Option<String> {
        match self {
            Channel::VoiceChannel { name, .. }
            | Channel::TextChannel { name, .. }
            | Channel::Group { name, .. } => Some(name.clone()),
            _ => None,
        }
    }
}

/// Partial values of [Channel]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PartialChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
}

/// Optional fields on channel object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FieldsChannel {
    Description,
    Icon,
    DefaultPermissions,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditChannel {
    /// Channel name
    /// length min: 1, max: 32
    pub name: Option<String>,
    /// Channel description
    /// length min: 0, max: 1024
    pub description: Option<String>,
    /// Group owner
    pub owner: Option<String>,
    /// Icon
    ///
    /// Provide an Autumn attachment Id.
    /// length min: 1, max: 128
    pub icon: Option<String>,
    /// Whether this channel is age-restricted
    pub nsfw: Option<bool>,
    /// Whether this channel is archived
    pub archived: Option<bool>,
    /// length min: 1
    pub remove: Option<Vec<FieldsChannel>>,
}
impl_to_vec!(DataEditChannel);

impl DataEditChannel {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self.to_owned()
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self.to_owned()
    }
    pub fn set_owner(&mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self.to_owned()
    }
    pub fn set_icon(&mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.to_owned()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
    pub fn set_archived(&mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self.to_owned()
    }

    pub fn add_remove(&mut self, channel: FieldsChannel) -> Self {
        match self.remove.clone() {
            Some(mut channel_vec) => {
                channel_vec.push(channel);
                self.remove = Some(channel_vec);
            }
            None => self.remove = Some(vec![channel]),
        }
        self.to_owned()
    }
}
