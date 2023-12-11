use serde::{Deserialize, Serialize};
use crate::impl_to_vec;

/// Web Push subscription
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebPushSubscription {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

/// Session information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,

    /// User Id
    pub user_id: String,

    /// Session token
    pub token: String,

    /// Display name
    pub name: String,

    /// Web Push subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<WebPushSubscription>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataEditSession {
    pub friendly_name: String,
}
impl_to_vec!(DataEditSession);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEditSession {
    _id: String,
    name: String,
}
