use reywen_http::results::{result, DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    structures::{
        channels::{channel::Channel, channel_invite::Invite},
        media::attachment::File,
        server::{
            server::{Category, FieldsServer, Server, SystemMessageChannels},
            server_ban::ServerBan,
        },
        users::user::User,
    },
};

impl Client {
    pub async fn server_ack(&self, server: &str) -> Result<(), DeltaError> {
        result(self.http.put(&format!("/servers/{server}/ack"), None).await).await
    }
    pub async fn server_create(
        &self,
        data: &DataCreateServer,
    ) -> Result<CreateServerResponse, DeltaError> {
        result(
            self.http
                .post(
                    &format!("/servers/create"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn server_delete(&self, server: &str) -> Result<(), DeltaError> {
        result(self.http.delete(&format!("/servers/{server}"), None).await).await
    }

    pub async fn server_edit(
        &self,
        server: &str,
        data: &DataEditServer,
    ) -> Result<Server, DeltaError> {
        result(
            self.http
                .patch(
                    &format!("/servers/{server}"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }

    pub async fn server_fetch(&self, server: &str) -> Result<Server, DeltaError> {
        result(self.http.get(&format!("/servers/{server}")).await).await
    }

    pub async fn ban_create(
        &self,
        server: &str,
        user: &str,
        reason: &DataBanReason,
    ) -> Result<DataBan, DeltaError> {
        let data = serde_json::to_string(&reason).unwrap();
        result(
            self.http
                .put(&format!("/servers/{server}/bans/{user}"), Some(&data))
                .await,
        )
        .await
    }
    pub async fn ban_list(&self, server: &str) -> Result<DataBanList, DeltaError> {
        result(self.http.get(&format!("/servers/{server}/bans")).await).await
    }

    pub async fn ban_remove(&self, server: &str, user: &str) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(&format!("/servers/{server}/bans/{user}"), None)
                .await,
        )
        .await
    }
    pub async fn channel_create(
        &self,
        server: &str,
        data: &DataChannelCreate,
    ) -> Result<Channel, DeltaError> {
        result(
            self.http
                .post(
                    &format!("/servers/{server}/channels"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }

    pub async fn invites_fetch(&self, server: &str) -> Result<Vec<Invite>, DeltaError> {
        result(self.http.get(&format!("/servers/{server}/invites")).await).await
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelCreate {
    #[serde(rename = "type")]
    pub r#type: Option<ChannelType>,
    pub name: String,
    pub description: Option<String>,
    pub nsfw: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
}

impl DataChannelCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            r#type: Some(ChannelType::Text),
            ..Default::default()
        }
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

impl DataCreateServer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
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
#[derive(Serialize, Deserialize, Debug, Default)]
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
    discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    analytics: Option<bool>,

    /// Fields to remove from server object
    remove: Option<Vec<FieldsServer>>,
}

impl DataEditServer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBanReason {
    pub reason: Option<String>,
}

impl DataBanReason {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: Some(String::from(reason)),
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
