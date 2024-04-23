use crate::structures::server::ban::{DataBan, DataBanReason};
use crate::structures::server::DataBanList;
use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::server::member::{DataMemberEdit, Member, ResponseMemberAll},
};
use crate::structures::server::member::MemberWithRoles;

impl Client {
    pub async fn member_edit(
        &self,
        server: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataMemberEdit>,
    ) -> Result<DataMemberEdit> {
        self.http
            .request(
                Method::PATCH,
                format!("/servers/{server}/members/{member}"),
                data.into(),
            )
            .await
    }
    pub async fn member_fetch(
        &self,
        server: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
    ) -> Result<Member> {
        self.http
            .request(
                Method::GET,
                format!("/servers/{server}/members/{member}"),
                None,
            )
            .await
    }
    pub async fn member_fetch_roles(
        &self,
        server: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
    ) -> Result<MemberWithRoles> {
        self.http
            .request(
                Method::GET,
                format!("/servers/{server}/members/{member}?roles=true"),
                None,
            )
            .await
    }
    pub async fn member_fetch_all(
        &self,
        server: impl Into<String> + std::fmt::Display,
    ) -> Result<ResponseMemberAll> {
        self.http
            .request(Method::GET, format!("/servers/{server}/members"), None)
            .await
    }
    pub async fn member_remove(
        &self,
        server: impl Into<String> + std::fmt::Display,
        member: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/servers/{server}/members/{member}"),
                None,
            )
            .await
    }

    pub async fn ban_create(
        &self,
        server: impl Into<String> + std::fmt::Display,
        user: impl Into<String> + std::fmt::Display,
        reason: impl Into<DataBanReason>,
    ) -> Result<DataBan> {
        self.http
            .request(
                Method::PUT,
                format!("/servers/{server}/bans/{user}"),
                &reason.into(),
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
}
