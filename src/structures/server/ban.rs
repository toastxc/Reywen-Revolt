use serde::{Deserialize, Serialize};

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
