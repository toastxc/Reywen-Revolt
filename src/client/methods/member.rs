use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::server::member::{DataMemberEdit, Member, ResponseMemberAll},
};

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
}
