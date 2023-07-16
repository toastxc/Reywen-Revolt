#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::structures::authentication::login::DataLogin;
    use crate::tests::tester_user;

    #[tokio::test]
    async fn test_session_logout() {
        let mut client = tester_user();
        client.session_logout().await.unwrap();
    }

    #[tokio::test]
    async fn test_session_login2() {
        let data = DataLogin::non_mfa("email", "password").set_friendly_name("hewoo");

        let _client = Client::from_login(&data).await.unwrap();
    }
}
