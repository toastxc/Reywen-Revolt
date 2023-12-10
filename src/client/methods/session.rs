use std::fmt::Debug;

use crate::reywen_http::{driver::Method, results::DeltaError, utils::struct_to_url, Delta};
use crate::{
    client::Client,
    json,
    structures::authentication::{
        login::{DataLogin, ResponseLogin},
        mfa::MFAResponse,
        session::SessionInfo,
    },
};
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn session_login(
        data: &DataLogin,
        url: Option<&str>,
    ) -> Result<ResponseLogin, DeltaError> {
        Delta::new()
            .set_url(url.unwrap_or("https://api.revolt.chat"))
            .request::<ResponseLogin>(Method::POST, "/auth/session/login", json!(data))
            .await
    }

    pub async fn session_logout(&mut self) -> Result<Client, DeltaError> {
        self.http
            .request::<()>(Method::GET, "/auth/session/logout", None)
            .await?;
        //self.http.headers.remove("x-session-token");
        self.http.header_delete("x-session-token");
        Ok(self.to_owned())
    }

    pub async fn session_fetch_all(&self) -> Result<Vec<SessionInfo>, DeltaError> {
        self.http
            .request(Method::GET, "/auth/session/all", None)
            .await
    }

    pub async fn session_delete_all(&self, revoke_self: bool) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/auth/session/all{}", struct_to_url(revoke_self)),
                None,
            )
            .await
    }

    pub async fn session_delete(&self, session_id: &str) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::DELETE,
                &format!("/auth/session/{session_id})"),
                None,
            )
            .await
    }

    pub async fn session_edit(
        &self,
        session_id: &str,
        friendly_name: &str,
    ) -> Result<(), DeltaError> {
        self.http
            .request(
                Method::PATCH,
                &format!("/auth/session/{session_id})"),
                json!(DataEditSession {
                    friendly_name: String::from(friendly_name),
                }),
            )
            .await
    }
}

impl Client {
    pub async fn session_login_smart(
        email: &str,
        password: &str,
        mfa_response: Option<MFAResponse>,
        friendly_name: Option<&str>,
    ) -> Result<ResponseLogin, DeltaError> {
        let original =
            Client::session_login(&DataLogin::email(email, password, friendly_name), None).await;

        if let Ok(ResponseLogin::MFA {
            ticket: mfa_ticket, ..
        }) = original
        {
            Client::session_login(
                &DataLogin::mfa(mfa_ticket, mfa_response, friendly_name),
                None,
            )
            .await
        } else {
            original
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataEditSession {
    friendly_name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseEditSession {
    _id: String,
    name: String,
}
