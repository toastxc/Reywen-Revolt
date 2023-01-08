use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMessage {
    
    pub _id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,

    pub channel: String,

    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Masquerade {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,

}


// skip embed

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMessagePayload {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
      
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<RReplies>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,

}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RReplies {
    pub id: String,
    pub mention: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RChannelFetch {
    pub channel_type: String,
    pub _id: String,
    pub server: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<RChannelIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<RPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_permissions: Option<RPermissionRoles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RChannelIcon {
    pub _id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: FileType,
    pub content_type: String,
    pub size: u32,
    pub deleted: bool,
    pub reported: bool,
    pub message_id: String,
    pub user_id: String,
    pub server_id: String,
    pub object_id: String,

}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RPermissions {
    pub a: u32,
    pub d: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RPermissionRoles {
    pub property1: RPermissions,
    pub property2: RPermissions,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileType {
    #[serde(rename = "type")]
    pub typer: String,
}

