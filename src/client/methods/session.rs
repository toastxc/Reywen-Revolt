use crate::{
    client::Client,
    json,
    structures::authentication::{
        login::{DataLogin, ResponseLogin},
        session::{Session, SessionInfo},
    },
};
use reywen_http::{driver::Method, results::DeltaError, utils::struct_to_url, Delta};
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn session_login(
        data: &DataLogin,
        url: Option<&str>,
    ) -> Result<ResponseLogin, AuthError> {
        match Delta::new()
            .set_url(url.unwrap_or("api.revolt.chat"))
            .request::<ResponseLogin>(Method::POST, "auth/session/login", json!(data))
            .await
        {
            Ok(response) => match response {
                ResponseLogin::Success(_) => Ok(response),
                ResponseLogin::MFA { .. } | ResponseLogin::Disabled { .. } => {
                    Err(AuthError::Auth(response))
                }
            },
            Err(reywen_error) => Err(AuthError::Http(reywen_error)),
        }
    }

    pub async fn session_logout(&mut self) -> Result<Client, DeltaError> {
        self.http
            .request::<()>(Method::GET, "/auth/session/logout", None)
            .await?;
        self.http.headers.remove("x-session-token");
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
    pub async fn from_login(data: &DataLogin) -> Result<Client, AuthError> {
        Self::from_login_url(data, None).await
    }

    pub async fn from_login_url(data: &DataLogin, url: Option<&str>) -> Result<Client, AuthError> {
        if let ResponseLogin::Success(Session { token, .. }) =
            Self::session_login(data, url).await?
        {
            return Ok(Client::from_token(&token, false).unwrap());
        };

        unreachable!()
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

#[derive(Debug)]
pub enum AuthError {
    Http(DeltaError),
    Auth(ResponseLogin),
}
