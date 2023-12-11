use crate::{
    client::{Client, Result},
    reywen_http::{driver::Method, utils::struct_to_url},
    structures::channels::message::{
        BulkMessageResponse, DataBulkDelete, DataEditMessage, DataMessageSearch, DataMessageSend,
        DataQueryMessages, DataUnreact, Message,
    },
};
use serde::Serialize;

impl Client {
    pub async fn message_ack(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::PUT,
                format!("/channels/{channel}/ack/{message}"),
                None,
            )
            .await
    }
    //  pub async fn message_acknowledge

    pub async fn message_bulk_delete(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataBulkDelete>,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/channels/{channel}/messages/bulk"),
                data.into(),
            )
            .await
    }
    pub async fn message_reaction_remove_all(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/channels/{channel}/messages/{message}/reactions"),
                None,
            )
            .await
    }

    pub async fn message_delete(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/channels/{channel}/messages/{message}"),
                None,
            )
            .await
    }
    pub async fn message_edit(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataEditMessage>,
    ) -> Result<Message> {
        self.http
            .request(
                Method::PATCH,
                format!("/channels/{channel}/messages/{message}"),
                data.into(),
            )
            .await
    }

    pub async fn message_fetch(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
    ) -> Result<Message> {
        self.http
            .request(
                Method::GET,
                format!("/channels/{channel}/messages/{message}"),
                None,
            )
            .await
    }

    pub async fn message_query(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        query: impl Into<&DataQueryMessages> + Serialize,
    ) -> Result<BulkMessageResponse> {
        self.http
            .request(
                Method::GET,
                format!("/channels/{channel}/messages{}", struct_to_url(query)),
                None,
            )
            .await
    }

    pub async fn message_reaction_add(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
        emoji: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(
                Method::PUT,
                format!("/channels/{channel}/messages/{message}/reactions/{emoji}"),
                None,
            )
            .await
    }
    pub async fn message_search(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataMessageSearch>,
    ) -> Result<BulkMessageResponse> {
        self.http
            .request(
                Method::POST,
                format!("/channels/{channel}/search"),
                data.into(),
            )
            .await
    }

    pub async fn message_send(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataMessageSend>,
    ) -> Result<Message> {
        self.http
            .request(
                Method::POST,
                format!("/channels/{channel}/messages"),
                data.into(),
            )
            .await
    }
    pub async fn message_reaction_remove(
        &self,
        channel: impl Into<String> + std::fmt::Display,
        message: impl Into<String> + std::fmt::Display,
        emoji: impl Into<String> + std::fmt::Display,
        data: impl Into<&DataUnreact>,
    ) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!(
                    "/channels/{channel}/messages/{message}/reactions/{emoji}{}",
                    struct_to_url(data.into())
                ),
                None,
            )
            .await
    }
}
