use reywen_http::results::{result, DeltaError};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    structures::{
        channels::channel::Channel,
        users::user::{FieldsUser, User, UserProfile, UserStatus},
    },
};

impl Client {
    pub async fn user_edit(&self, user: &str, data: &DataEditUser) -> Result<User, DeltaError> {
        let data = serde_json::to_string(&data).unwrap();

        result(self.http.patch(&format!("users/{user}"), Some(&data)).await).await
    }
    pub async fn user_fetch(&self, user: &str) -> Result<User, DeltaError> {
        result(self.http.get(&format!("users/{user}")).await).await
    }
    pub async fn user_profile_fetch(&self, user: &str) -> Result<Vec<UserProfile>, DeltaError> {
        result(self.http.get(&format!("/users/{user}/profile")).await).await
    }

    pub async fn fetch_mutual(&self, user: &str) -> Result<MutualResponse, DeltaError> {
        result(self.http.get(&format!("users/{user}/mutual")).await).await
    }

    pub async fn user_fetch_self(&self) -> Result<User, DeltaError> {
        result(self.http.get("users/@me").await).await
    }

    pub async fn user_block_remove(&self, user: &str) -> Result<User, DeltaError> {
        result(
            self.http
                .delete(&format!("/users/{user}/block"), None)
                .await,
        )
        .await
    }

    pub async fn dm_open(&self, user: &str) -> Result<Channel, DeltaError> {
        result(self.http.get(&format!("users/{user}/dm")).await).await
    }

    pub async fn dm_fetch_all(&self) -> Result<Vec<Channel>, DeltaError> {
        result(self.http.get("users/friend").await).await
    }
    pub async fn default_avatar_fetch(&self, user: &str) -> Result<String, DeltaError> {
        result(self.http.get(&format!("users/{user}/default_avatar")).await).await
    }
    pub async fn user_flags_fetch(&self, user: &str) -> Result<ResponseFlag, DeltaError> {
        result(self.http.get(&format!("users/{user}")).await).await
    }

    pub async fn user_block(&self, user: &str) -> Result<User, DeltaError> {
        result(self.http.post(&format!("/users/{user}/block"), None).await).await
    }

    pub async fn friend_request_send(&self, username: &str) -> Result<User, DeltaError> {
        result(
            self.http
                .post(
                    "/users/friend",
                    Some(
                        &serde_json::to_string(&DataSendFriendRequest::set_username(username))
                            .unwrap(),
                    ),
                )
                .await,
        )
        .await
    }
    pub async fn friend_request_accept(&self, user: &str) -> Result<User, DeltaError> {
        result(self.http.put(&format!("/users/{user}/friend"), None).await).await
    }

    pub async fn friend_request_reject(&self, user: &str) -> Result<User, DeltaError> {
        result(
            self.http
                .delete(&format!("/users/{user}/friend"), None)
                .await,
        )
        .await
    }
    pub async fn friend_remove(&self, user: &str) -> Result<User, DeltaError> {
        self.friend_request_reject(user).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSendFriendRequest {
    pub username: String,
}
impl DataSendFriendRequest {
    pub fn set_username(username: &str) -> Self {
        Self {
            username: String::from(username),
        }
    }
}

//https://api.revolt.chat/users/{target}
/// # User Data
#[derive(Serialize, Debug, Clone, Default)]
pub struct DataEditUser {
    /// Attachment Id for avatar
    pub avatar: Option<String>,
    /// New user status
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    pub profile: Option<UserProfileData>,
    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// Enum of user flags
    pub flags: Option<i32>,
    /// Fields to remove from user object
    pub remove: Option<Vec<FieldsUser>>,
}

impl DataEditUser {
    pub fn set_avatar(&mut self, avatar: &str) -> Self {
        self.avatar = Some(String::from(avatar));
        self.to_owned()
    }
    pub fn set_status(&mut self, status: UserStatus) -> Self {
        self.status = Some(status);
        self.to_owned()
    }
    pub fn set_profile(&mut self, profile: UserProfileData) -> Self {
        self.profile = Some(profile);
        self.to_owned()
    }
    pub fn set_badges(&mut self, badges: i32) -> Self {
        self.badges = Some(badges);
        self.to_owned()
    }
    pub fn set_flags(&mut self, flags: i32) -> Self {
        self.flags = Some(flags);
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: Vec<FieldsUser>) -> Self {
        self.remove = Some(remove);
        self.to_owned()
    }
    pub fn add_remove(&mut self, remove: FieldsUser) -> Self {
        match self.remove.clone() {
            Some(mut data) => {
                data.push(remove);
                self.remove = Some(data);
            }
            None => self.remove = Some(vec![remove]),
        }
        self.to_owned()
    }

    pub fn new() -> Self {
        Default::default()
    }
}
#[derive(Serialize, Debug, Clone, Default)]
pub struct UserProfileData {
    /// Text to set as user profile description
    content: Option<String>,
    /// Attachment Id for background
    background: Option<String>,
}

impl UserProfileData {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_content(&mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.to_owned()
    }
    pub fn set_background(&mut self, background: &str) -> Self {
        self.background = Some(String::from(background));
        self.to_owned()
    }
}
/// # Mutual Friends and Servers Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct MutualResponse {
    /// Array of mutual user IDs that both users are friends with
    pub users: Vec<String>,
    /// Array of mutual server IDs that both users are in
    pub servers: Vec<String>,
}

/// # Flag Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseFlag {
    /// Flags
    pub flags: i32,
}
