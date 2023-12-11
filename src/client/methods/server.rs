use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::{
        channels::{invite::Invite, Channel},
        server::{
            CreateServerResponse, DataBan, DataBanList, DataBanReason, DataChannelCreate,
            DataCreateServer, DataEditServer, Server,
        },
    },
};

impl Client {
    pub async fn server_ack(&self, server: impl Into<String> + std::fmt::Display) -> Result<()> {
        self.http
            .request(Method::PUT, format!("/servers/{server}/ack"), None)
            .await
    }
    pub async fn server_create(
        &self,
        data: impl Into<&DataCreateServer>,
    ) -> Result<CreateServerResponse> {
        self.http
            .request(Method::POST, "/servers/create", data.into())
            .await
    }
    pub async fn server_delete(&self, server: &str) -> Result<()> {
        self.http
            .request(Method::DELETE, format!("/servers/{server}"), None)
            .await
    }

    pub async fn server_edit(
        &self,
        server: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditServer>,
    ) -> Result<Server> {
        self.http
            .request(Method::PATCH, &format!("/servers/{server}"), data.into())
            .await
    }

    pub async fn server_fetch(
        &self,
        server: impl Into<String> + std::fmt::Display,
    ) -> Result<Server> {
        self.http
            .request(Method::GET, &format!("/servers/{server}"), None)
            .await
    }

    pub async fn ban_create(
        &self,
        server: impl Into<String> + std::fmt::Display,
        user: impl Into<String> + std::fmt::Display,
        reason: impl Into<Option<String>>,
    ) -> Result<DataBan> {
        self.http
            .request(
                Method::PUT,
                format!("/servers/{server}/bans/{user}"),
                &DataBanReason {
                    reason: reason.into(),
                },
            )
            .await
    }
    pub async fn ban_list(
        &self,
        server: impl Into<String> + std::fmt::Display,
    ) -> Result<DataBanList> {
        self.http
            .request(Method::GET, format!("/servers/{server}/bans"), None)
            .await
    }

    pub async fn ban_remove(
        &self,
        server: impl Into<String> + std::fmt::Display,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/servers/{server}/bans/{user}"),
                None,
            )
            .await
    }

    pub async fn member_kick(&self, server: &str, user: &str) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/servers/{server}/members/{user}"),
                None,
            )
            .await
    }

    pub async fn channel_create(
        &self,
        server: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataChannelCreate>,
    ) -> Result<Channel> {
        self.http
            .request(
                Method::POST,
                format!("/servers/{server}/channels"),
                data.into(),
            )
            .await
    }

    pub async fn invites_fetch(
        &self,
        server: impl Into<String> + std::fmt::Display,
    ) -> Result<Vec<Invite>> {
        self.http
            .request(Method::GET, format!("/servers/{server}/invites"), None)
            .await
    }
}
