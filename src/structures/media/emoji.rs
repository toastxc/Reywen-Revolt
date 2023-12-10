use crate::reywen_http::utils::if_false;
use serde::{Deserialize, Serialize};

/// Information about what owns this emoji
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: String },
    Detached,
}

/// Representation of an Emoji on Revolt
#[derive(Serialize, Deserialize, Debug, Clone)]
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
