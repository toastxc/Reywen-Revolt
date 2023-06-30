//json

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::structures::{
    channels::{channel::Channel, message::Message},
    media::{attachment::File, embeds::Embed, emoji::Emoji},
    server::server::{Category, Role, Server, SystemMessageChannels},
    users::user::{User, UserStatus},
};

/// Ping Packet
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Ping {
    Binary(Vec<u8>),
    Number(u128),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebSocketEvent {
    Authenticated,
    Error {
        error: String,
    },
    Pong {
        data: usize,
    },
    Ready {
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>,
        emojis: Vec<Emoji>,
    },
    Message {
        #[serde(flatten)]
        message: Message,
    },
    MessageUpdate {
        #[serde(rename = "id")]
        message_id: String,
        #[serde(rename = "channel")]
        channel_id: String,

        data: MessageUpdateData,
    },
    MessageDelete {
        #[serde(rename = "id")]
        message_id: String,
        #[serde(rename = "channel")]
        channel_id: String,
    },
    MessageAppend {
        #[serde(rename = "id")]
        message_id: String,
        #[serde(rename = "channel")]
        channel_id: String,
        append: MessageAppendData,
    },
    ChannelCreate {
        #[serde(flatten)]
        channel: Channel,
    },
    ChannelUpdate {
        #[serde(rename = "id")]
        channel_id: String,
        data: ChannelUpdateData,
        clear: Vec<ChannelUpdateClear>,
    },
    ChannelDelete {
        #[serde(rename = "id")]
        channel_id: String,
    },
    ChannelGroupJoin {
        #[serde(rename = "id")]
        channel_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelGroupLeave {
        #[serde(rename = "id")]
        channel_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelStartTyping {
        #[serde(rename = "id")]
        channel_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelStopTyping {
        #[serde(rename = "id")]
        channel_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ChannelAck {
        #[serde(rename = "id")]
        channel_id: String,
        #[serde(rename = "user")]
        user_id: String,
        message_id: String,
    },
    ServerUpdate {
        #[serde(rename = "id")]
        server_id: String,
        data: ServerUpdateData,
        clear: Vec<ServerUpdateClear>,
    },
    ServerDelete {
        #[serde(rename = "id")]
        server_id: String,
    },
    ServerMemberUpdate {
        #[serde(rename = "id")]
        member_id: String,
        data: ServerMemberUpdateData,
        clear: Vec<ServerMemberUpdateClear>,
    },
    ServerMemberJoin {
        #[serde(rename = "id")]
        server_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ServerMemberLeave {
        #[serde(rename = "id")]
        server_id: String,
        #[serde(rename = "user")]
        user_id: String,
    },
    ServerRoleUpdate {
        #[serde(rename = "id")]
        server_id: String,
        role_id: String,
        data: ServerRoleUpdateData,
        clear: Vec<ServerRoleUpdateClear>,
    },
    ServerRoleDelete {
        #[serde(rename = "id")]
        server_id: String,
        role_id: String,
    },
    UserUpdate {
        #[serde(rename = "id")]
        user_id: String,
        data: UserUpdateData,
        clear: Vec<UserUpdateClear>,
    },
    UserRelationship {
        id: String,
        #[serde(rename = "user")]
        user_id: String,
        status: Value,
    },
    MessageReact {
        #[serde(rename = "id")]
        message_id: String,
        channel_id: String,
        user_id: String,
        emoji_id: String,
    },
    MessageUnreact {
        #[serde(rename = "id")]
        message_id: String,
        channel_id: String,
        user_id: String,
        emoji_id: String,
    },
    MessageRemoveReaction {
        #[serde(rename = "id")]
        message_id: String,
        channel_id: String,
        emoji_id: String,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChannelUpdateData {
    pub name: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChannelUpdateClear {
    Icon,
    Description,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerUpdateData {
    pub owner: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<File>,
    pub banner: Option<File>,
    pub default_permissions: Option<u64>,
    pub nsfw: Option<bool>,
    pub system_messages: Option<SystemMessageChannels>,
    pub categories: Option<HashMap<String, Category>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerUpdateClear {
    Icon,
    Banner,
    Description,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerMemberUpdateData {
    pub nickname: Option<String>,
    pub avatar: Option<File>,
    pub roles: Option<Vec<Role>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerRoleUpdateData {
    pub name: Option<String>,
    pub colour: Option<String>,
    pub hoist: Option<bool>,
    pub rank: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerRoleUpdateClear {
    Colour,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerMemberUpdateClear {
    Nickname,
    Avatar,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserUpdateData {
    pub status: Option<UserStatus>,

    #[serde(rename = "profile.background")]
    pub profile_background: Option<File>,

    #[serde(rename = "profile.content")]
    pub profile_content: Option<String>,

    pub avatar: Option<File>,
    pub online: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserUpdateClear {
    ProfileContent,
    ProfileBackground,
    StatusText,
    Avatar,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageUpdateData {
    pub edited: String,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageAppendData {
    #[serde(default)]
    embeds: Option<Vec<Embed>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebSocketSend {
    Authenticate { token: String },
    BeginTyping { channel: String },
    EndTyping { channel: String },
    Ping { data: usize },
}

impl WebSocketSend {
    pub fn authenticate(token: &str) -> Self {
        WebSocketSend::Authenticate {
            token: String::from(token),
        }
    }
    pub fn typing(channel: &str, status: bool) -> Self {
        match status {
            true => Self::BeginTyping {
                channel: String::from(channel),
            },
            false => Self::EndTyping {
                channel: String::from(channel),
            },
        }
    }
    pub fn ping(data: usize) -> Self {
        WebSocketSend::Ping { data }
    }
}
