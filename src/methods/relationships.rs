use serde::{Deserialize, Serialize};

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
pub async fn fetch_mutal_servers_and_friends(domain: &str, token: &str) -> Option<MutualResponse> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/users/dm"))
        .header("x-bot-token", token)
        .send()
        .await
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
pub async fn accept_friend(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .put(format!("https://{domain}/users/{user}/friend"))
        .header("x-bot-token", token)
        .send()
        .await
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
pub async fn deny_friend(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .delete(format!("https://{domain}/users/{user}/friend"))
        .header("x-bot-token", token)
        .send()
        .await
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
pub async fn block_user(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .put(format!("https://{domain}/users/{user}/block"))
        .header("x-bot-token", token)
        .send()
        .await
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
pub async fn unblock_user(domain: &str, token: &str, user: &str) -> Option<User> {
    match reqwest::Client::new()
        .delete(format!("https://{domain}/users/{user}/block"))
        .header("x-bot-token", token)
        .send()
        .await
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
#[derive(Serialize, Deserialize)]
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
    data: DataSendFriendRequest,
) -> Option<User> {
    match reqwest::Client::new()
        .delete(format!("https://{domain}/users/friend"))
        //https://api.revolt.chat/users/friend
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
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
