#[cfg(test)]
mod tests {

    use crate::tests::tester_bot;

    #[tokio::test]
    async fn experiment() {
        let client = tester_bot();

        client
            .channel_fetch_exper("01H48GMVZ5923GMC79TWQJ99W6")
            .await
            .unwrap();
    }
}
