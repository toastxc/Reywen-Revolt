use reywen_http::{driver::Method, results::DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    json,
    structures::{server::server_member::Member, users::user::User},
};

impl Client {
    pub async fn member_edit(
        &self,
        server: &str,
        member: &str,
        data: &DataMemberEdit,
    ) -> Result<DataMemberEdit, DeltaError> {
        self.http
            .request(
                Method::PATCH,
                &format!("/servers/{server}/members/{member}"),
                json!(data),
            )
            .await
    }
    pub async fn member_fetch(&self, server: &str, member: &str) -> Result<Member, DeltaError> {
        self.http
            .request(
                Method::GET,
                &format!("/servers/{server}/members/{member}"),
                None,
            )
            .await
    }
    pub async fn member_fetch_all(&self, server: &str) -> Result<ResponseMemberAll, DeltaError> {
        self.http
            .request(Method::GET, &format!("/servers/{server}/members"), None)
            .await
    }
    pub async fn member_remove(&self, server: &str, member: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/servers/{server}/members/{member}"),
                None,
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
