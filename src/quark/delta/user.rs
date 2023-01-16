use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RUserFetch {
    pub _id: String,
    pub username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Avatar>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Avatar {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub metadata_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Bot {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Avatar>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Relation {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Status {
    pub text: String,
    pub presence: String,
}
