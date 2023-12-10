use super::origin;
use crate::reywen_http::{driver::Method, results::DeltaError, utils::struct_to_url};
use crate::{
    client::{methods::opt_vec_add, Client},
    json, opt_str, ref_str,
    structures::channels::message::{
        BulkMessageResponse, Interactions, Masquerade, Message, MessageSort, Reply, SendableEmbed,
    },
};
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn message_ack(&self, channel: &str, message: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/channels/{channel}/ack/{message}"),
                None,
            )
            .await
    }

    pub async fn message_bulk_delete(
        &self,
        channel: &str,
        data: &DataBulkDelete,
    ) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/channels/{channel}/messages/bulk"),
                json!(data),
            )
            .await
    }
    pub async fn message_reaction_remove_all(
        &self,
        channel: &str,
        message: &str,
    ) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/channels/{channel}/messages/{message}/reactions"),
                None,
            )
            .await
    }

    pub async fn message_delete(&self, channel: &str, message: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/channels/{channel}/messages/{message}"),
                None,
            )
            .await
    }
    pub async fn message_edit(
        &self,
        channel: &str,
        message: &str,
        data: &DataEditMessage,
    ) -> Result<Message, DeltaError> {
        self.http
            .request(
                Method::PATCH,
                &format!("/channels/{channel}/messages/{message}"),
                json!(data),
            )
            .await
    }

    pub async fn message_fetch(&self, channel: &str, message: &str) -> Result<Message, DeltaError> {
        self.http
            .request(
                Method::GET,
                &format!("/channels/{channel}/messages/{message}"),
                None,
            )
            .await
    }

    pub async fn message_query(
        &self,
        channel: &str,
        query: &DataQueryMessages,
    ) -> Result<BulkMessageResponse, DeltaError> {
        self.http
            .request(
                Method::GET,
                &format!("/channels/{channel}/messages{}", struct_to_url(query)),
                None,
            )
            .await
    }

    pub async fn message_reaction_add(
        &self,
        channel: &str,
        message: &str,
        emoji: &str,
    ) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::PUT,
                &format!("/channels/{channel}/messages/{message}/reactions/{emoji}"),
                None,
            )
            .await
    }
    pub async fn message_search(
        &self,
        channel: &str,
        data: &DataMessageSearch,
    ) -> Result<BulkMessageResponse, DeltaError> {
        self.http
            .request(
                Method::POST,
                &format!("/channels/{channel}/search"),
                json!(data),
            )
            .await
    }

    pub async fn message_send(
        &self,
        channel: &str,
        data: &DataMessageSend,
    ) -> Result<Message, DeltaError> {
        self.http
            .request(
                Method::POST,
                &format!("/channels/{channel}/messages"),
                json!(data),
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
        self.http
            .request(
                Method::DELETE,
                &format!(
                    "/channels/{channel}/messages/{message}/reactions/{emoji}{}",
                    struct_to_url(data)
                ),
                None,
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
        Default::default()
    }
    pub fn add_message(&mut self, message: &str) -> Self {
        self.ids.push(String::from(message));
        self.to_owned()
    }
    pub fn set_messages(&mut self, mut messages: Vec<&str>) -> Self {
        messages
            .iter_mut()
            .for_each(|item| self.ids.push(item.to_string()));
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
        Default::default()
    }

    pub fn set_content(&mut self, content: &str) -> Self {
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
    pub include_users: Option<bool>,
}

impl DataQueryMessages {
    pub fn new() -> Self {
        Default::default()
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
    pub fn set_include_users(&mut self, inclue_users: bool) -> Self {
        self.include_users = Some(inclue_users);
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
    pub fn set_attachments(&mut self, attachments: Vec<String>) -> Self {
        self.attachments = origin(&self.attachments, attachments);
        self.to_owned()
    }
    pub fn add_attachment(&mut self, attachment: &str) -> Self {
        self.attachments = opt_vec_add(&self.attachments, ref_str!(attachment));
        self.to_owned()
    }

    pub fn set_replies(&mut self, replies: Vec<Reply>) -> Self {
        self.replies = origin(&self.replies, replies);
        self.to_owned()
    }
    pub fn add_reply(&mut self, reply: &Reply) -> Self {
        self.replies = opt_vec_add(&self.replies, reply);
        self.to_owned()
    }
    pub fn set_reply_str(&mut self, replies: Vec<String>) {
        self.replies = Some(
            replies
                .into_iter()
                .map(|id| Reply {
                    id,
                    ..Default::default()
                })
                .collect(),
        )
    }
    pub fn add_reply_str(&mut self, reply: &str) -> Self {
        self.add_reply(&Reply {
            id: String::from(reply),
            ..Default::default()
        });
        self.to_owned()
    }

    pub fn set_embeds(&mut self, embeds: Vec<SendableEmbed>) -> Self {
        let embeds = origin(&self.embeds, embeds);
        self.embeds = embeds;
        self.to_owned()
    }
    pub fn add_embed(&mut self, embed: &SendableEmbed) -> Self {
        self.embeds = opt_vec_add(&self.embeds, embed);
        self.to_owned()
    }

    pub fn set_masquerade(&mut self, masquerade: &Masquerade) -> Self {
        self.masquerade = Some(masquerade.to_owned());
        self.to_owned()
    }

    pub fn set_interactions(&mut self, interactions: Interactions) -> Self {
        self.interactions = Some(interactions);
        self.to_owned()
    }
    pub fn add_reaction(&mut self, reaction: &str) -> Self {
        self.set_interactions(
            self.interactions
                .as_ref()
                .map_or(Interactions::new(), |origin| origin.to_owned())
                .add_reaction(reaction),
        )
    }
    pub fn set_reactions(&mut self, reactions: Vec<String>) -> Self {
        let mut interactions = self.interactions.clone().unwrap_or_default();
        reactions.into_iter().for_each(|item| {
            interactions.add_reaction(&item);
        });

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
        Default::default()
    }
    pub fn set_user_id(&mut self, user_id: &str) -> Self {
        self.user_id = opt_str!(user_id);
        self.to_owned()
    }
    pub fn set_remove_all(&mut self, remove_all: bool) -> Self {
        self.remove_all = Some(remove_all);
        self.to_owned()
    }
}
