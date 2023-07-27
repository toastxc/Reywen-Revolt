#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::structures::authentication::mfa::MFAResponse;
    use crate::tests::tester_user;

    #[tokio::test]
    async fn test_session_logout() {
        let mut client = tester_user();
        client.session_logout().await.unwrap();
    }

    #[tokio::test]
    async fn test_session_login2() {
        let a = Client::session_login_smart("", "", Some(MFAResponse::totp("")))
            .await
            .unwrap();
        panic!("{:#?}", a);
    }

    #[tokio::test]
    async fn test_session_delete() {
        tester_user().session_delete("").await.unwrap();
    }
}
