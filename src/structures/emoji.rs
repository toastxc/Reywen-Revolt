use crate::impl_to_vec;
use crate::reywen_http::utils::if_false;
use serde::{Deserialize, Serialize};
/// Emoji
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Emoji {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// What owns this emoji
    pub parent: EmojiParent,
    /// Uploader user id
    pub creator_id: String,
    /// Emoji name
    pub name: String,
    /// Whether the emoji is animated

    #[serde(skip_serializing_if = "if_false", default)]
    pub animated: bool,
    /// Whether the emoji is marked as nsfw
    #[serde(skip_serializing_if = "if_false", default)]
    pub nsfw: bool,
}

/// Create a new emoji
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DataCreateEmoji {
    /// Server name
    pub name: String,
    /// Parent information
    pub parent: EmojiParent,
    /// Whether the emoji is mature
    #[serde(default)]
    pub nsfw: bool,
}

impl DataCreateEmoji {
    pub fn new(name: impl Into<String>, server_id: impl Into<String>) -> Self {
        DataCreateEmoji {
            name: name.into(),
            parent: EmojiParent::Server {
                id: server_id.into(),
            },
            nsfw: false,
        }
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self.clone()
    }
    pub fn set_server(&mut self, server_id: impl Into<String>) -> Self {
        self.parent = EmojiParent::Server {
            id: server_id.into(),
        };
        self.clone()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self.clone()
    }
}
impl_to_vec!(DataCreateEmoji);
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: String },

    Detached,
}
