use reywen_http::{driver::Method, results::DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    json,
    structures::{channels::Channel, users::User},
};

impl Client {
    pub async fn group_member_add(&self, group: &str, member: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::PATCH,
                &format!("/channels/{group}/recipients/{member}"),
                None,
            )
            .await
    }

    pub async fn group_create(&self, data: &DataCreateGroup) -> Result<Channel, DeltaError> {
        self.http
            .request(Method::POST, "channels/create", json!(data))
            .await
    }
    pub async fn group_member_remove(&self, channel: &str, member: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/channels/{channel}/recipients/{member}"),
                None,
            )
            .await
    }

    pub async fn group_member_fetch_all(&self, channel: &str) -> Result<Vec<User>, DeltaError> {
        self.http
            .request(Method::GET, &format!("/channels/{channel}/members"), None)
            .await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataCreateGroup {
    /// Group name (min: 1, max: 32)
    pub name: String,
    /// Group description (length min: 0, max: 1024)
    pub description: Option<String>,
    /// Array of user IDs to add to the group
    ///
    /// Must be friends with these users.
    /// Length min: 0, max: 49
    pub users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl DataCreateGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self.to_owned()
    }

    pub fn add_user(&mut self, user: &str) -> Self {
        self.users.push(String::from(user));
        self.to_owned()
    }
    pub fn set_users(&mut self, users: Vec<&str>) -> Self {
        let mut user_vec = Vec::new();
        for x in users {
            user_vec.push(String::from(x));
        }
        self.users = user_vec;
        self.to_owned()
    }

    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
}
