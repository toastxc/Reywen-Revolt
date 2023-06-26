use reywen_http::{
    results::{result, DeltaError},
    utils::struct_to_url,
};
use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    structures::channels::message::{
        BulkMessageResponse2, Interactions, Masquerade, Message, MessageSort, Reply, SendableEmbed,
    },
};

impl Client {
    pub async fn message_ack(&self, channel: &str, message: &str) -> Result<(), DeltaError> {
        result(
            self.http
                .put(&format!("/channels/{channel}/ack/{message}"), None)
                .await,
        )
        .await
    }

    pub async fn message_bulk_delete(
        &self,
        channel: &str,
        messages: &DataBulkDelete,
    ) -> Result<(), DeltaError> {
        let data = serde_json::to_string(messages).unwrap();
        result(
            self.http
                .delete(&format!("/channels/{channel}/messages/bulk"), Some(&data))
                .await,
        )
        .await
    }
    pub async fn message_reaction_remove_all(
        &self,
        channel: &str,
        message: &str,
    ) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(
                    &format!("/channels/{channel}/messages/{message}/reactions"),
                    None,
                )
                .await,
        )
        .await
    }

    pub async fn message_delete(&self, channel: &str, message: &str) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(&format!("/channels/{channel}/messages/{message}"), None)
                .await,
        )
        .await
    }
    pub async fn message_edit(
        &self,
        channel: &str,
        message: &str,
        edit_data: &DataEditMessage,
    ) -> Result<Message, DeltaError> {
        let data = serde_json::to_string(edit_data).unwrap();
        result(
            self.http
                .patch(
                    &format!("/channels/{channel}/messages/{message}"),
                    Some(&data),
                )
                .await,
        )
        .await
    }

    pub async fn message_fetch(&self, channel: &str, message: &str) -> Result<Message, DeltaError> {
        result(
            self.http
                .get(&format!("/channels/{channel}/messages/{message}"))
                .await,
        )
        .await
    }

    pub async fn message_query(
        &self,
        channel: &str,
        query: &DataQueryMessages,
    ) -> Result<BulkMessageResponse2, DeltaError> {
        result(
            self.http
                .get(&format!(
                    "/channels/{channel}/messages{}",
                    struct_to_url(query)
                ))
                .await,
        )
        .await
    }
    pub async fn message_reaction_add(
        &self,
        channel: &str,
        message: &str,
        emoji: &str,
    ) -> Result<(), DeltaError> {
        result(
            self.http
                .put(
                    &format!("/channels/{channel}/messages/{message}/reactions/{emoji}"),
                    None,
                )
                .await,
        )
        .await
    }
    pub async fn message_search(
        &self,
        channel: &str,
        search_options: &DataMessageSearch,
    ) -> Result<BulkMessageResponse2, DeltaError> {
        let data = serde_json::to_string(search_options).unwrap();
        result(
            self.http
                .post(&format!("/channels/{channel}/search"), Some(&data))
                .await,
        )
        .await
    }

    pub async fn message_send(
        &self,
        channel: &str,
        message: &DataMessageSend,
    ) -> Result<Message, DeltaError> {
        let data = serde_json::to_string(message).unwrap();
        result(
            self.http
                .post(&format!("/channels/{channel}/messages"), Some(&data))
                .await,
        )
        .await
    }
    pub async fn message_reaction_remove(
        &self,
        channel: &str,
        message: &str,
        emoji: &str,
        data: &DataUnreact,
    ) -> Result<(), DeltaError> {
        result(
            self.http
                .delete(
                    &format!(
                        "/channels/{channel}/messages/{message}/reactions/{emoji}{}",
                        struct_to_url(data)
                    ),
                    None,
                )
                .await,
        )
        .await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataBulkDelete {
    /// Message IDs
    pub ids: Vec<String>,
}

impl DataBulkDelete {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn add_message(&mut self, message: &str) -> Self {
        self.ids.push(String::from(message));
        self.to_owned()
    }
    pub fn set_messages(&mut self, messages: Vec<&str>) -> Self {
        for x in messages {
            self.ids.push(String::from(x))
        }
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditMessage {
    /// New message content (length min: 1, length max: 2000)
    content: Option<String>,
    /// Embeds to include in the message (length min: 0, length max: 10)
    embeds: Option<Vec<SendableEmbed>>,
}
impl DataEditMessage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn content(&mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.to_owned()
    }
}

/// # Query Parameters
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataQueryMessages {
    /// Maximum number of messages to fetch
    ///
    /// For fetching nearby messages, this is \`(limit + 1)\`.
    /// min: 1, max 100
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    pub after: Option<String>,
    /// Message sort direction
    pub sort: Option<MessageSort>,
    /// Message id to search around
    ///
    /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
    /// It will also take half of limit rounded as the limits to each side.
    /// It also fetches the message ID specified.
    /// length min: 26, max: 26
    pub nearby: Option<String>,
    /// Whether to include user (and member, if server channel) objects
    pub include_users: Option<bool>,
}

impl DataQueryMessages {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_limit(&mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self.to_owned()
    }

    pub fn set_before(&mut self, before: &str) -> Self {
        self.before = Some(String::from(before));
        self.to_owned()
    }
    pub fn set_after(&mut self, after: &str) -> Self {
        self.after = Some(String::from(after));
        self.to_owned()
    }

    pub fn set_sort(&mut self, sort: MessageSort) -> Self {
        self.sort = Some(sort);
        self.to_owned()
    }
    pub fn set_nearby(&mut self, nearby: &str) -> Self {
        self.nearby = Some(String::from(nearby));
        self.to_owned()
    }

    pub fn set_include_users(&mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self.to_owned()
    }
}

/// # Search Parameters
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataMessageSearch {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    /// length min: 1, max: 64
    pub query: String,

    /// Maximum number of messages to fetch
    /// length min: 1, max: 100
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    pub after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by latest.
    pub sort: MessageSort,
    /// Whether to include user (and member, if server channel) objects
    pub include_users: Option<bool>,
}

impl DataMessageSearch {
    pub fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            ..Default::default()
        }
    }
    pub fn set_limit(&mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self.to_owned()
    }
    pub fn set_before(&mut self, before: &str) -> Self {
        self.before = Some(String::from(before));
        self.to_owned()
    }
    pub fn set_after(&mut self, after: &str) -> Self {
        self.after = Some(String::from(after));
        self.to_owned()
    }
    pub fn set_sort(&mut self, sort: MessageSort) -> Self {
        self.sort = sort;
        self.to_owned()
    }
    pub fn set_include_users(&mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataMessageSend {
    /// Message content to send
    /// length min: 0, max: 2000
    pub content: Option<String>,
    /// Attachments to include in message
    /// length min: 1, max: 128
    pub attachments: Option<Vec<String>>,
    /// Messages to reply to
    pub replies: Option<Vec<Reply>>,
    /// Embeds to include in message
    ///
    /// Text embed content contributes to the content length cap
    /// length min: 1, max: 10
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Masquerade to apply to this message
    pub masquerade: Option<Masquerade>,
    /// Information about how this message should be interacted with
    pub interactions: Option<Interactions>,
}

impl DataMessageSend {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_content(&mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.to_owned()
    }

    pub fn set_masquerade(&mut self, masquerade: &Masquerade) -> Self {
        self.masquerade = Some(masquerade.clone());
        self.to_owned()
    }
}

/// # Query Parameters
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataUnreact {
    /// Remove a specific user's reaction
    pub user_id: Option<String>,
    /// Remove all reactions
    pub remove_all: Option<bool>,
}

impl DataUnreact {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_user_id(&mut self, user_id: &str) -> Self {
        self.user_id = Some(String::from(user_id));
        self.to_owned()
    }
    pub fn set_remove_all(&mut self, remove_all: bool) -> Self {
        self.remove_all = Some(remove_all);
        self.to_owned()
    }
}
