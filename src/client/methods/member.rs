use iso8601_timestamp::Timestamp;
use reywen_http::{driver::Method, results::DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    json, ref_str,
    structures::{
        server::member::{FieldsMember, Member},
        users::User,
    },
};

use super::opt_vec_add;

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
pub struct ResponseMemberAll {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}
/// # Member Data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataMemberEdit {
    /// Member nickname
    nickname: Option<String>,
    /// Attachment Id to set for avatar
    avatar: Option<String>,
    /// Array of role ids
    roles: Option<Vec<String>>,
    /// Timestamp this member is timed out until
    timeout: Option<Timestamp>,
    /// Fields to remove from channel object
    remove: Option<Vec<FieldsMember>>,
}

impl DataMemberEdit {
    pub fn set_nickname(&mut self, nickname: &str) -> Self {
        self.nickname = Some(String::from(nickname));
        self.to_owned()
    }
    pub fn set_avatar(&mut self, avatar: &str) -> Self {
        self.avatar = Some(String::from(avatar));
        self.to_owned()
    }
    pub fn set_roles(&mut self, roles: Vec<String>) -> Self {
        self.roles = Some(roles);
        self.to_owned()
    }
    pub fn add_role(&mut self, role: &str) -> Self {
        self.roles = opt_vec_add(&self.roles, ref_str!(role));
        self.to_owned()
    }
    pub fn set_timeout(&mut self, timeout: Timestamp) -> Self {
        self.timeout = Some(timeout);
        self.to_owned()
    }

    pub fn add_remove(&mut self, remove: &FieldsMember) -> Self {
        self.remove = opt_vec_add(&self.remove, remove);
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: Vec<FieldsMember>) -> Self {
        self.remove = Some(remove);
        self.to_owned()
    }
}
