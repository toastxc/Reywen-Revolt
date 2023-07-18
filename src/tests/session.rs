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
        let data2 =
            Client::session_login_smart("EMAIL", "PASSSWORD", Some(MFAResponse::totp("CODE")))
                .await;

        println!("DATA: {:#?}", data2)
    }
}
