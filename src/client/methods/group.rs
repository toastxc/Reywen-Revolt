use crate::{
    client::Client,
    client::Result,

    reywen_http::driver::Method,
    structures::{channels::Channel, users::User},
};
use crate::structures::channels::group::DataCreateGroup;

impl Client {
    pub async fn group_member_add(
        &self,
        group: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::PATCH,
                format!("/channels/{group}/recipients/{member}"),
                None,
            )
            .await
    }

    pub async fn group_create(&self, data: impl Into<&DataCreateGroup>) -> Result<Channel> {
        self.http
            .request(Method::POST, "/channels/create", data.into())
            .await
    }
    pub async fn group_member_remove(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/channels/{channel}/recipients/{member}"),
                None,
            )
            .await
    }

    pub async fn group_member_fetch_all(
        &self,
        channel: impl Into<String> + std::fmt::Display,
    ) -> Result<Vec<User>> {
        self.http
            .request(Method::GET, format!("/channels/{channel}/members"), None)
            .await
    }
}
