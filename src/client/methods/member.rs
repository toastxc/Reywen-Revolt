use reywen_http::results::{result, DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    structures::{server::server_member::Member, users::user::User},
};

impl Client {
    pub async fn member_edit(
        &self,
        server: &str,
        member: &str,
        data: &DataMemberEdit,
    ) -> Result<DataMemberEdit, DeltaError> {
        result(
            self.http
                .patch(
                    &format!("/servers/{server}/members/{member}"),
                    Some(&serde_json::to_string(&data).unwrap()),
                )
                .await,
        )
        .await
    }
    pub async fn member_fetch(&self, server: &str, member: &str) -> Result<Member, DeltaError> {
        result(
            self.http
                .get(&format!("/servers/{server}/members/{member}"))
                .await,
        )
        .await
    }
    pub async fn member_fetch_all(&self, server: &str) -> Result<ResponseMemberAll, DeltaError> {
        result(self.http.get(&format!("/servers/{server}/members")).await).await
    }
    pub async fn member_remove(&self, server: &str, member: &str) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(&format!("/servers/{server}/members/{member}"), None)
                .await,
        )
        .await
    }
}

/// # Member List
///
/// Both lists are sorted by ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllMemberResponse {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}

/// # Member List
///
/// Both lists are sorted by ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMemberAll {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataMemberEdit {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub roles: Option<Vec<String>>,
    pub timeout: Option<String>,
    pub remove: Option<Vec<String>>,
}
