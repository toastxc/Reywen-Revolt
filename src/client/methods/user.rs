use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::{
        channels::Channel,
        users::{
            DataEditUser, DataSendFriendRequest, MutualResponse, ResponseFlag, User, UserProfile,
        },
    },
};

impl Client {
    pub async fn user_edit(
        &self,
        user: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditUser>,
    ) -> Result<User> {
        self.http
            .request(Method::PATCH, format!("/users/{user}"), data.into())
            .await
    }
    pub async fn user_fetch(&self, user: impl Into<String> + std::fmt::Display) -> Result<User> {
        self.http
            .request(Method::GET, format!("/users/{user}"), None)
            .await
    }
    pub async fn user_profile_fetch(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<Vec<UserProfile>> {
        self.http
            .request(Method::GET, format!("/users/{user}/profile"), None)
            .await
    }

    pub async fn fetch_mutual(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<MutualResponse> {
        self.http
            .request(Method::GET, format!("/users/{user}/mutual"), None)
            .await
    }

    pub async fn user_fetch_self(&self) -> Result<User> {
        self.http.request(Method::GET, "/users/@me", None).await
    }

    pub async fn user_block_remove(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<User> {
        self.http
            .request(Method::DELETE, format!("/users/{user}/block"), None)
            .await
    }

    pub async fn dm_open(&self, user: &str) -> Result<Channel> {
        self.http
            .request(Method::GET, format!("/users/{user}/dm"), None)
            .await
    }

    pub async fn dm_fetch_all(&self) -> Result<Vec<Channel>> {
        self.http.request(Method::GET, "/users/dms", None).await
    }
    pub async fn default_avatar_fetch(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<Vec<u8>> {
        self.http
            .request_raw(Method::GET, format!("/users/{user}/default_avatar"), None)
            .await
    }
    pub async fn user_flags_fetch(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<ResponseFlag> {
        self.http
            .request(Method::GET, &format!("/users/{user}/flags"), None)
            .await
    }

    pub async fn user_block(&self, user: impl Into<String> + std::fmt::Display) -> Result<User> {
        self.http
            .request(Method::PUT, format!("/users/{user}/block"), None)
            .await
    }

    pub async fn friend_request_send(
        &self,
        username: impl Into<String> + std::fmt::Display,
    ) -> Result<User> {
        self.http
            .request(
                Method::POST,
                "/users/friend",
                &DataSendFriendRequest::set_username(&username.into()),
            )
            .await
    }
    pub async fn friend_request_accept(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<User> {
        self.http
            .request(Method::PUT, format!("/users/{user}/friend"), None)
            .await
    }

    pub async fn friend_request_reject(
        &self,
        user: impl Into<String> + std::fmt::Display,
    ) -> Result<User> {
        self.http
            .request(Method::DELETE, format!("/users/{user}/friend"), None)
            .await
    }
    pub async fn friend_remove(&self, user: impl Into<String> + std::fmt::Display) -> Result<User> {
        self.friend_request_reject(user).await
    }
}
