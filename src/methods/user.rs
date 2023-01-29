use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    debug::Web,
    structs::user::{FieldsUser, User, UserStatus},
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
pub async fn user_fetch(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "fetch_channel");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn user_edit(domain: &str, token: &str, user: &str, edit: DataEditUser) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&edit).unwrap())
        .send()
        .await
    {
        Web::error(e, "fetch_channel");
    }
}
#[allow(dead_code)]
pub async fn user_default_avatar(domain: &str, token: &str, user: &str) -> Option<String> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/{user}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "fetch_channel");
            None
        }
        Ok(a) => Some(a.text().await.unwrap()),
    }
}


