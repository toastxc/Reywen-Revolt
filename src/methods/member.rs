use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    client::Web,
    structs::{server::Member, user::User},
};

// ################################## SERVER MEMBERS ##################################
#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub struct AllMemberResponse {
    /// List of members
    members: Vec<Member>,
    /// List of users
    users: Vec<User>,
}
/// Polls server for members - None for failure
#[allow(dead_code)]
pub async fn fetch_all(
    domain: &str,
    token: &str,
    header: &str,
    server: &str,
) -> Option<Vec<Member>> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/servers/{server}/members"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "server_fetch_members");
            None
        }
        Ok(a) => match serde_json::from_str::<Vec<Member>>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
/// Polls server for a member - None for failure
#[allow(dead_code)]
pub async fn fetch(
    domain: &str,
    token: &str,
    header: &str,
    server: &str,
    member: &str,
) -> Option<Member> {
    match reqwest::Client::new()
        .get(format!(
            "https://{domain}/servers/{server}/members/{member}"
        ))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Err(http_err) => {
            Web::error(http_err, "server_fetch_member");
            None
        }
        Ok(a) => match serde_json::from_str::<Member>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

#[allow(dead_code)]
pub async fn kick(domain: &str, token: &str, header: &str, server: &str, member: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!(
            "https://{domain}/servers/{server}/members/{member}"
        ))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "server_kick_member");
    };
}
/// # Member Data
#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataMemberEdit {
    /// Member nickname
    #[validate(length(min = 1, max = 32))]
    pub nickname: Option<String>,
    /// Attachment Id to set for avatar
    pub avatar: Option<String>,
    /// Array of role ids
    pub roles: Option<Vec<String>>,
    /// Timestamp this member is timed out until
    pub timeout: Option<Timestamp>,
    /// Fields to remove from channel object
    #[validate(length(min = 1))]
    pub remove: Option<Vec<FieldsMember>>,
}
/// Optional fields on server member object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsMember {
    Nickname,
    Avatar,
    Roles,
    Timeout,
}

#[allow(dead_code)]
pub async fn edit(
    domain: &str,
    token: &str,
    header: &str,
    server: &str,
    member: &str,
    edit: DataMemberEdit,
) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/server/{server}/members/{member}"))
        .header(header, token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&edit).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "server_edit_member");
    };
}
/// # Ban Information
#[derive(Validate, Serialize, Deserialize)]
pub struct DataBanCreate {
    /// Ban reason
    #[validate(length(min = 1, max = 1024))]
    reason: Option<String>,
}
impl DataBanCreate {
    pub fn new(reason: Option<&str>) -> DataBanCreate {
        DataBanCreate {
            reason: reason.map(String::from),
        }
    }
}

#[allow(dead_code)]
pub async fn ban(
    domain: &str,
    token: &str,
    header: &str,
    server: &str,
    member: &str,
    reason: DataBanCreate,
) {
    if let Err(e) = reqwest::Client::new()
        .put(format!("https://{domain}/servers/{server}/bans/{member}"))
        .header(header, token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&reason).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "server_ban_member");
    };
}

#[allow(dead_code)]
pub async fn unban(domain: &str, token: &str, header: &str, server: &str, member: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/servers/{server}/bans/{member}"))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "server_unban_member");
    };
}
