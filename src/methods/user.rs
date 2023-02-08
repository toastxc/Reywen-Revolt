use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    client::Web,
    structs::{
        channel::Channel,
        user::{FieldsUser, User, UserProfile, UserStatus},
    },
};

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct UserProfileData {
    /// Text to set as user profile description
    #[validate(length(min = 0, max = 2000))]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    /// Attachment Id for background
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 128))]
    background: Option<String>,
}

/// # User Data
#[derive(Validate, Serialize, Deserialize)]
pub struct DataEditUser {
    /// New user status
    #[validate]
    status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    #[validate]
    profile: Option<UserProfileData>,
    /// Attachment Id for avatar
    #[validate(length(min = 1, max = 128))]
    avatar: Option<String>,
    /// Fields to remove from user object
    #[validate(length(min = 1))]
    remove: Option<Vec<FieldsUser>>,
}
#[allow(dead_code)]
pub async fn fetch_self(domain: &str, token: &str) -> Option<User> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/@me"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "user_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn fetch(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "user_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn edit(domain: &str, token: &str, user: &str, edit: DataEditUser) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&edit).unwrap())
        .send()
        .await
    {
        Web::error(e, "user_edit");
    }
}
#[derive(Serialize, Deserialize)]
pub struct FlagResponse {
    /// Flags
    flags: i32,
}

#[allow(dead_code)]
pub async fn fetch_flags(domain: &str, token: &str, user: &str) -> Option<FlagResponse> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}/flags"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "user_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<FlagResponse>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

/// # Username Information
#[derive(Validate, Serialize, Deserialize)]
pub struct DataChangeUsername {
    /// New username
    #[validate(length(min = 2, max = 32))]
    username: String,
    /// Current account password
    #[validate(length(min = 8, max = 1024))]
    password: String,
}

impl DataChangeUsername {
    #[allow(dead_code)]
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    #[allow(dead_code)]
    pub fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }
    #[allow(dead_code)]
    pub fn password(mut self, username: String) -> Self {
        self.username = username;
        self
    }
}

#[allow(dead_code)]
pub async fn change_username(domain: &str, token: &str, user: &str, data: DataChangeUsername) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
    {
        Web::error(e, "change_username");
    }
}

#[allow(dead_code)]
pub async fn fetch_default_avatar(domain: &str, token: &str, user: &str, data: DataChangeUsername) {
    if let Err(e) = reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}/default_avatar"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
    {
        Web::error(e, "fetch_default_avatar");
    }
}

#[allow(dead_code)]
pub async fn fetch_user_profile(domain: &str, token: &str, user: &str) -> Option<UserProfile> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}/default_avatar"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(e) => {
            Web::error(e, "server_create");
            None
        }
        Ok(a) => match serde_json::from_str::<UserProfile>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn fetch_dm_channels(domain: &str, token: &str) -> Vec<Channel> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/dms"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(e) => {
            Web::error(e, "fetch_dms");
            Vec::new()
        }
        Ok(a) => match serde_json::from_str::<Vec<Channel>>(&a.text().await.unwrap()) {
            Err(_) => Vec::new(),
            Ok(a) => a,
        },
    }
}

#[allow(dead_code)]
pub async fn open_dm(domain: &str, token: &str, user: &str) -> Vec<Channel> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}/dm"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(e) => {
            Web::error(e, "open_dm");
            Vec::new()
        }
        Ok(a) => match serde_json::from_str::<Vec<Channel>>(&a.text().await.unwrap()) {
            Err(_) => Vec::new(),
            Ok(a) => a,
        },
    }
}
