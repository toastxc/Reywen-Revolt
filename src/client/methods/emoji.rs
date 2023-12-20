use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::emoji::{DataCreateEmoji, Emoji},
};
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn emoji_create(
        &self,
        data: impl Into<&DataCreateEmoji>,
        emoji_id: impl Into<String> + std::fmt::Display,
    ) -> Result<Emoji> {
        self.http
            .request(Method::PUT, format!("/custom/emoji/{emoji_id}"), data.into())
            .await
    }
    pub async fn emoji_delete(
        &self,
        emoji_id: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(Method::DELETE, format!("/custom/emoji/{emoji_id}"), None)
            .await
    }
    pub async fn emoji_fetch(&self, emoji_id: impl Into<String> + std::fmt::Display) -> Result<Emoji> {
        self.http
            .request(Method::GET, format!("/custom/emoji/{emoji_id}"), None)
            .await
    }
}

#[derive(Serialize, Deserialize)]
pub struct Reference {
    /// Id of object
    pub id: String,
}
