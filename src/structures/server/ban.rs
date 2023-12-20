use serde::{Deserialize, Serialize};
use crate::impl_to_vec;

use super::member::MemberCompositeKey;

/// Representation of a server ban on Revolt
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerBan {
    /// Unique member id
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,
    /// Reason for ban creation
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataBanReason {
    pub reason: Option<String>,
}
impl_to_vec!(DataBanReason);

impl From<Option<String>> for DataBanReason {
    fn from(value: Option<String>) -> Self {
        DataBanReason {
            reason: value.map(|reason| reason.to_string()),
        }
    }
}

impl From<String> for DataBanReason {
    fn from(value: String) -> Self {
        DataBanReason {
            reason: Some(value),
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
