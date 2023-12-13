use crate::{
    client::{Client, Result},
    reywen_http::driver::Method,
    structures::users::bot::{Bot, PublicBot},
};

use crate::structures::users::bot::{
    BotResponse, DataBotInvite, DataCreateBot, DataEditBot, OwnedBotsResponse,
};

impl Client {
    pub async fn bot_create(&self, data: impl Into<&DataCreateBot>) -> Result<Bot> {
        self.http
            .request(Method::POST, "/bots/create", data.into())
            .await
    }
    pub async fn bot_delete(&self, bot_id: impl Into<String> + std::fmt::Display) -> Result<()> {
        self.http
            .request(Method::DELETE, format!("/bots/{bot_id}"), None)
            .await
    }

    pub async fn bot_edit(
        &self,
        bot_id: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditBot>,
    ) -> Result<Bot> {
        self.http
            .request(Method::PATCH, format!("/bots/{bot_id}"), data.into())
            .await
    }
    pub async fn bot_fetch(
        &self,
        bot_id: impl Into<String> + std::fmt::Display,
    ) -> Result<BotResponse> {
        self.http
            .request(Method::GET, format!("/bots/{bot_id}"), None)
            .await
    }
    pub async fn bot_fetch_owned(&self) -> Result<OwnedBotsResponse> {
        self.http.request(Method::GET, "/bots/@me", None).await
    }
    pub async fn bot_fetch_public(
        &self,
        bot_id: impl Into<String> + std::fmt::Display,
    ) -> Result<PublicBot> {
        self.http
            .request(Method::GET, format!("/bots/{bot_id}/invite"), None)
            .await
    }

    pub async fn bot_invite(
        &self,
        bot_id: impl Into<String> + std::fmt::Display,
        server_or_group: impl Into<String> + std::fmt::Display,
        is_server: bool,
    ) -> Result<()> {
        self.http
            .request(
                Method::POST,
                format!("/bots/{bot_id}/invite"),
                &(match (is_server, server_or_group.into()) {
                    (true, server) => DataBotInvite::Server { server },
                    (false, group) => DataBotInvite::Group { group },
                }),
            )
            .await
    }
}
