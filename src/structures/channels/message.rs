use crate::client::methods::{opt_vec_add, origin};
use crate::impl_to_vec;
use crate::reywen_http::utils::if_false;
use crate::structures::{
    media::{attachment::File, embeds::Embed},
    server::member::Member,
    users::User,
};
use indexmap::{IndexMap, IndexSet};
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Reply {
    /// Message Id
    pub id: String,
    /// Whether this reply should mention the message's author
    pub mention: bool,
}
/// Representation of a text embed before it is sent.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SendableEmbed {
    pub icon_url: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub colour: Option<String>,
}

impl SendableEmbed {
    pub fn set_icon_url(&mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self.clone()
    }

    pub fn set_url(&mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self.clone()
    }
    pub fn set_title(&mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self.clone()
    }
    pub fn set_description(&mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self.clone()
    }
    pub fn set_media(&mut self, media: impl Into<String>) -> Self {
        self.media = Some(media.into());
        self.clone()
    }
    pub fn set_colour(&mut self, colour: impl Into<String>) -> Self {
        self.colour = Some(colour.into());
        self.clone()
    }
    pub fn set_color(&mut self, color: impl Into<String>) -> Self {
        self.set_colour(color)
    }
}

/// Representation of a system event message
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemMessage {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: String, by: String },
    #[serde(rename = "user_remove")]
    UserRemove { id: String, by: String },
    #[serde(rename = "user_joined")]
    UserJoined { id: String },
    #[serde(rename = "user_left")]
    UserLeft { id: String },
    #[serde(rename = "user_kicked")]
    UserKicked { id: String },
    #[serde(rename = "user_banned")]
    UserBanned { id: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: String },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: String },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: String },
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged { from: String, to: String },
}

/// Name and / or avatar override information
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Replace the display role colour shown on this message
    ///
    /// Must have `ManageRole` permission to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

impl Masquerade {
    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_avatar(&mut self, avatar: &str) -> Self {
        self.avatar = Some(String::from(avatar));
        self.to_owned()
    }
    pub fn set_color(&mut self, color: &str) -> Self {
        self.colour = Some(String::from(color));
        self.to_owned()
    }
    pub fn set_colour(&mut self, color: &str) -> Self {
        self.colour = Some(String::from(color));
        self.to_owned()
    }
    pub fn new() -> Self {
        Default::default()
    }
}

/// Information to guide interactions on this message
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Interactions {
    /// Reactions which should always appear and be distinct
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<IndexSet<String>>,
    /// Whether reactions should be restricted to the given list
    ///
    /// Can only be set to true if reactions list is of at least length 1
    #[serde(skip_serializing_if = "if_false")]
    pub restrict_reactions: bool,
}

impl Interactions {
    pub fn is_default(&self) -> bool {
        self.reactions.is_none()
    }
    pub fn add_reaction(&mut self, reaction: &str) -> Self {
        let mut origin = match self.reactions.clone() {
            Some(original) => original,
            None => IndexSet::new(),
        };
        origin.insert(String::from(reaction));
        self.reactions = Some(origin);
        self.to_owned()
    }

    pub fn set_reactions(&mut self, reactions: Vec<String>) -> Self {
        let mut index = self.reactions.to_owned().unwrap_or_default();
        reactions.into_iter().for_each(|item| {
            index.insert(item);
        });
        self.reactions = Some(index);
        self.to_owned()
    }
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Unique value generated by client sending this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Id of the channel this message was sent in
    pub channel: String,
    /// Id of the user that sent this message
    pub author: String,
    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// System message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMessage>,
    /// Array of attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<File>>,
    /// Time at which this message was last edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<Timestamp>,
    /// Attached embeds to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    /// Array of user ids mentioned in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    /// Array of message ids this message is replying to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub reactions: IndexMap<String, IndexSet<String>>,
    /// Information about how this message should be interacted with
    #[serde(skip_serializing_if = "Interactions::is_default", default)]
    pub interactions: Interactions,
    /// Name and / or avatar overrides for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
}

impl Message {
    pub fn content_is(&self, input: &str) -> bool {
        self.content.to_owned().unwrap_or_default().as_str() == input
    }
    pub fn content_contains(&self, search: &str, split_by: &str) -> Option<Vec<String>> {
        if let Some(content) = self.content.as_ref() {
            let split_content = content.split(split_by).collect::<Vec<&str>>();
            let mut new = Vec::new();
            split_content
                .iter()
                .for_each(|item| new.push(item.to_string()));

            if split_content.contains(&search) {
                return Some(new);
            }
            None
        } else {
            None
        }
    }
}

/// # Message Sort
///
/// Sort used for retrieving messages
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum MessageSort {
    /// Sort by the most relevant messages
    #[default]
    Relevance,
    /// Sort by the newest messages first
    Latest,
    /// Sort by the oldest messages first
    Oldest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageTimePeriod {
    Relative {
        /// Message id to search around
        ///
        /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
        /// It will also take half of limit rounded as the limits to each side.
        /// It also fetches the message ID specified.
        nearby: String,
    },
    Absolute {
        /// Message id before which messages should be fetched
        before: Option<String>,
        /// Message id after which messages should be fetched
        after: Option<String>,
        /// Message sort direction
        sort: Option<MessageSort>,
    },
}

/// # Bulk Message Response
///
/// Response used when multiple messages are fetched
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BulkMessageResponse {
    JustMessages(
        /// List of messages
        Vec<Message>,
    ),
    MessagesAndUsers {
        /// List of messages
        messages: Vec<Message>,
        /// List of users
        users: Vec<User>,
        /// List of members
        #[serde(skip_serializing_if = "Option::is_none")]
        members: Option<Vec<Member>>,
    },
}
impl_to_vec!(BulkMessageResponse);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendMessage {
    /// Additional embeds to include in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataBulkDelete {
    /// Message IDs
    pub ids: Vec<String>,
}
impl_to_vec!(DataBulkDelete);

impl From<Vec<String>> for DataBulkDelete {
    fn from(value: Vec<String>) -> Self {
        Self { ids: value }
    }
}
impl DataBulkDelete {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_message(&mut self, message: impl Into<String>) -> Self {
        self.ids.push(message.into());
        self.to_owned()
    }
    pub fn set_messages(&mut self, messages: impl Into<Vec<String>>) -> Self {
        messages
            .into()
            .iter_mut()
            .for_each(|item| self.ids.push(item.to_string()));
        self.to_owned()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditMessage {
    /// New message content (length min: 1, length max: 2000)
    pub content: Option<String>,
    /// Embeds to include in the message (length min: 0, length max: 10)
    pub embeds: Option<Vec<SendableEmbed>>,
}
impl_to_vec!(DataEditMessage);
impl DataEditMessage {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_content(content: impl Into<String> + std::fmt::Display) -> Self {
        Self::default().set_content(content)
    }
    pub fn from_embed_text(embed: impl Into<String>) -> Self {
        Self::default().add_embed_text(embed)
    }

    pub fn from_embed(embed: impl Into<SendableEmbed>) -> Self {
        Self::default().add_embed(embed)
    }

    pub fn set_content(&mut self, content: impl Into<String> + std::fmt::Display) -> Self {
        self.content = Some(content.into());
        self.to_owned()
    }

    pub fn add_embed(&mut self, embed: impl Into<SendableEmbed>) -> Self {
        if let Some(mut a) = self.embeds.clone() {
            a.push(embed.into())
        }
        self.clone()
    }

    pub fn add_embed_text(&mut self, message: impl Into<String>) -> Self {
        if let Some(mut a) = self.embeds.clone() {
            a.push(SendableEmbed::default().set_title(message))
        }
        self.clone()
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
        self.clone()
    }

    pub fn set_before(&mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self.clone()
    }
    pub fn set_after(&mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self.clone()
    }

    pub fn set_sort(&mut self, sort: impl Into<MessageSort>) -> Self {
        self.sort = Some(sort.into());
        self.clone()
    }
    pub fn set_nearby(&mut self, nearby: impl Into<String>) -> Self {
        self.nearby = Some(nearby.into());
        self.clone()
    }
    pub fn set_include_users(&mut self, inclue_users: bool) -> Self {
        self.include_users = Some(inclue_users);
        self.clone()
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
impl_to_vec!(DataMessageSearch);
impl DataMessageSearch {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }
    pub fn set_limit(&mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self.clone()
    }
    pub fn set_before(&mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self.to_owned()
    }
    pub fn set_after(&mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self.clone()
    }
    pub fn set_sort(&mut self, sort: impl Into<MessageSort>) -> Self {
        self.sort = sort.into();
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
impl_to_vec!(DataMessageSend);
impl DataMessageSend {
    pub fn from_content(content: impl Into<String>) -> Self {
        Self::default().set_content(content)
    }
    pub fn from_embed(embed: SendableEmbed) -> Self {
        Self::default().add_embed(embed)
    }
    pub fn from_embed_text(embed_text: impl Into<String>) -> Self {
        Self::default().add_embed(SendableEmbed::default().set_title(embed_text.into()))
    }

    pub fn add_embed_text(&mut self, text: impl Into<String>) -> Self {
        self.add_embed(SendableEmbed::default().set_title(text))
    }
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_content(&mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self.clone()
    }
    pub fn set_attachments(&mut self, attachments: impl Into<Vec<String>>) -> Self {
        origin(&mut self.attachments, attachments.into());
        self.to_owned()
    }
    pub fn add_attachment(&mut self, attachment: impl Into<String>) -> Self {
        opt_vec_add(&mut self.attachments, attachment.into());
        self.to_owned()
    }

    pub fn set_replies(&mut self, replies: impl Into<Vec<Reply>>) -> Self {
        origin(&mut self.replies, replies.into());
        self.to_owned()
    }
    pub fn add_reply(&mut self, reply: impl Into<Reply>) -> Self {
        opt_vec_add(&mut self.replies, reply.into());
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
    pub fn add_reply_str(&mut self, reply: impl Into<String>) -> Self {
        self.add_reply(Reply {
            id: reply.into(),
            ..Default::default()
        });
        self.to_owned()
    }

    pub fn set_embeds(&mut self, embeds: impl Into<Vec<SendableEmbed>>) -> Self {
        origin(&mut self.embeds, embeds.into());

        self.clone()
    }
    pub fn add_embed(&mut self, embed: impl Into<SendableEmbed>) -> Self {
        opt_vec_add(&mut self.embeds, embed.into());
        self.to_owned()
    }

    pub fn set_masquerade(&mut self, masquerade: impl Into<Masquerade>) -> Self {
        self.masquerade = Some(masquerade.into());
        self.to_owned()
    }

    pub fn set_interactions(&mut self, interactions: impl Into<Interactions>) -> Self {
        self.interactions = Some(interactions.into());
        self.to_owned()
    }
    pub fn add_reaction(&mut self, reaction: impl Into<String>) -> Self {
        self.set_interactions(
            self.interactions
                .as_ref()
                .map_or(Interactions::new(), |origin| origin.to_owned())
                .add_reaction(&reaction.into()),
        )
    }
    pub fn set_reactions(&mut self, reactions: impl Into<Vec<String>> + Clone) -> Self {
        let mut interactions = self.interactions.clone().unwrap_or_default();
        reactions.into().clone().into_iter().for_each(|item| {
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
    pub fn set_user_id(&mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self.to_owned()
    }
    pub fn set_remove_all(&mut self, remove_all: bool) -> Self {
        self.remove_all = Some(remove_all);
        self.to_owned()
    }
}
