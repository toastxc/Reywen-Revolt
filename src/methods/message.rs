use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    client::Web,
    structs::message::{DataMessageSend, Message},
};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub enum MessageSort {
    /// Sort by the most relevant messages
    Relevance,
    /// Sort by the newest messages first
    Latest,
    /// Sort by the oldest messages first
    Oldest,
}
impl Default for MessageSort {
    fn default() -> MessageSort {
        MessageSort::Relevance
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub struct OptionsMessageSearch {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    #[validate(length(min = 1, max = 64))]
    query: String,

    /// Maximum number of messages to fetch
    #[validate(range(min = 1, max = 100))]
    limit: Option<i64>,
    /// Message id before which messages should be fetched
    #[validate(length(min = 26, max = 26))]
    before: Option<String>,
    /// Message id after which messages should be fetched
    #[validate(length(min = 26, max = 26))]
    after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by relevance.
    #[serde(default = "MessageSort::default")]
    sort: MessageSort,
    /// Whether to include user (and member, if server channel) objects
    include_users: Option<bool>,
}

/// # Message Details
#[derive(Validate, Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditMessage {
    /// New message content
    #[validate(length(min = 1, max = 2000))]
    content: Option<String>,
}
#[allow(dead_code)]
pub async fn fetch(domain: &str, token: &str, header: &str, channel: &str) -> Option<Vec<Message>> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/channels/{channel}/messages"))
        .header(header, token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "message_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<Vec<Message>>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}

pub async fn send(
    domain: &str,
    token: &str,
    header: &str,
    channel: &str,
    message: DataMessageSend,
) {
    if let Err(e) = reqwest::Client::new()
        .post(format!("https://{domain}/channels/{channel}/messages"))
        .header(header, token)
        .header("Idempotency-Key", rand::random::<u64>())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "message_send");
    };
}

#[allow(dead_code)]
pub async fn search(
    domain: &str,
    token: &str,
    header: &str,
    channel: &str,
    message: OptionsMessageSearch,
) {
    if let Err(e) = reqwest::Client::new()
        .post(format!("https://{domain}/channels/{channel}/search"))
        .header(header, token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "message_search");
    };
}
#[allow(dead_code)]
pub async fn delete(domain: &str, token: &str, header: &str, channel: &str, message: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!(
            "https://{domain}/channels/{channel}/messages/{message}"
        ))
        .header(header, token)
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "message_delete");
    };
}
#[allow(dead_code)]
pub async fn edit(
    domain: &str,
    token: &str,
    header: &str,
    channel: &str,
    message: &str,

    changes: DataEditMessage,
) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/channels/{channel}/{message}"))
        .header(header, token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&changes).unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
    {
        Web::error(e, "message_delete");
    };
}
