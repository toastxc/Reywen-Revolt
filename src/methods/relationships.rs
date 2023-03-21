use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{client::Web, structs::user::User};

/// # Mutual Friends and Servers Response
#[derive(Serialize, Deserialize)]
pub struct MutualResponse {
    /// Array of mutual user IDs that both users are friends with
    users: Vec<String>,
    /// Array of mutual server IDs that both users are in
    servers: Vec<String>,
}

#[allow(dead_code)]
pub async fn fetch_mutal_servers_and_friends(
    domain: &str,
    token: &str,
    header: &str,
) -> Option<MutualResponse> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/dm"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(e) => {
            Web::error(e, "mutal_server_and_users");
            None
        }
        Ok(a) => match serde_json::from_str::<MutualResponse>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn accept_friend(domain: &str, token: &str, header: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .put(format!("https://{domain}/users/{user}/friend"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "accept_friend");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
#[allow(dead_code)]
pub async fn deny_friend(domain: &str, token: &str, header: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .delete(format!("https://{domain}/users/{user}/friend"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "deny_friend");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn block(domain: &str, token: &str, header: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .put(format!("https://{domain}/users/{user}/block"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "block_user");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn unblock(domain: &str, token: &str, header: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .delete(format!("https://{domain}/users/{user}/block"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "unblock_user");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
/// # User Lookup Information
#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataSendFriendRequest {
    username: String,
}

impl DataSendFriendRequest {
    #[allow(dead_code)]
    pub fn new(username: &str) -> Self {
        Self {
            username: String::from(username),
        }
    }
}

#[allow(dead_code)]
pub async fn friend_request(
    domain: &str,
    token: &str,
    header: &str,
    data: DataSendFriendRequest,
) -> Option<User> {
    match reqwest::Client::new()
        .post(format!("https://{domain}/users/friend"))
        .header(header, token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "friend_request");
            None
        }
        Ok(a) => match serde_json::from_str::<User>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
