use crate::{
    client::{Client, Result},
    reywen_http::{driver::Method, utils::struct_to_url, Delta},
    structures::authentication::{
        login::{DataLogin, ResponseLogin},
        mfa::MFAResponse,
        session::{DataEditSession, SessionInfo},
    },
};

impl Client {
    pub async fn session_login(
        data: impl Into<&DataLogin>,
        url: impl Into<Option<String>>,
    ) -> Result<ResponseLogin> {
        Delta::new()
            .set_url(url.into().unwrap_or("https://api.revolt.chat".to_string()))
            .request::<ResponseLogin>(Method::POST, "/auth/session/login", data.into())
            .await
    }

    pub async fn session_logout(&mut self) -> Result<Client> {
        self.http
            .request::<()>(Method::GET, "/auth/session/logout", None)
            .await?;
        //self.http.headers.remove("x-session-token");
        self.http.header_delete("x-session-token");
        Ok(self.to_owned())
    }

    pub async fn session_fetch_all(&self) -> Result<Vec<SessionInfo>> {
        self.http
            .request(Method::GET, "/auth/session/all", None)
            .await
    }

    pub async fn session_delete_all(&self, revoke_self: bool) -> Result<()> {
        self.http
            .request(
                Method::DELETE,
                format!("/auth/session/all{}", struct_to_url(revoke_self)),
                None,
            )
            .await
    }

    pub async fn session_delete(
        &self,
        session_id: impl Into<String> + std::fmt::Display,
    ) -> Result<()> {
        self.http
            .request(Method::DELETE, format!("/auth/session/{session_id})"), None)
            .await
    }

    pub async fn session_edit(
        &self,
        session_id: impl Into<String> + std::fmt::Display,
        friendly_name: impl Into<String>,
    ) -> Result<()> {
        self.http
            .request(
                Method::PATCH,
                format!("/auth/session/{session_id})"),
                &DataEditSession {
                    friendly_name: friendly_name.into(),
                },
            )
            .await
    }
}

impl Client {
    pub async fn session_login_smart(
        email: impl Into<String> + std::fmt::Display,
        password: impl Into<String> + std::fmt::Display,
        mfa_response: impl Into<Option<MFAResponse>>,
        friendly_name: impl Into<Option<String>> + Clone,
    ) -> Result<ResponseLogin> {
        let original = Client::session_login(
            &DataLogin::email(
                &email.into(),
                &password.into(),
                friendly_name.clone().into().as_deref(),
            ),
            None,
        )
        .await;

        if let Ok(ResponseLogin::MFA {
            ticket: mfa_ticket, ..
        }) = original
        {
            Client::session_login(
                &DataLogin::mfa(
                    mfa_ticket,
                    mfa_response.into(),
                    friendly_name.into().as_deref(),
                ),
                None,
            )
            .await
        } else {
            original
        }
    }
}
