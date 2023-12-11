use crate::{
    client::Client,
    client::Result,
    reywen_http::driver::Method,
    structures::channels::{invite::Invite, Channel, DataEditChannel},
};

impl Client {
    pub async fn channel_delete(
        &self,
        channel: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(Method::DELETE, format!("/channels/{channel}"), None)
            .await
    }
    pub async fn channel_edit(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditChannel>,
    ) -> Result<Channel> {
        self.http
            .request(Method::PATCH, format!("/channels/{channel}"), data.into())
            .await
    }
    pub async fn channel_fetch(
        &self,
        channel: impl Into<String> + std::fmt::Display,
    ) -> Result<Channel> {
        self.http
            .request(Method::GET, format!("/channels/{channel}"), None)
            .await
    }

    pub async fn channel_invite_create(
        &self,
        channel: impl Into<String> + std::fmt::Display,
    ) -> Result<Invite> {
        self.http
            .request(Method::POST, format!("/channels/{channel}/invites"), None)
            .await
    }
}
