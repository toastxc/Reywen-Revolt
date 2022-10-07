use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMessage {
    
    pub _id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,

    pub channel: String,

    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Masquerade {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,

}


// skip embed

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RMessagePayload {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
      
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<RReplies>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,

}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RReplies {
    pub id: String,
    pub mention: bool,
}
