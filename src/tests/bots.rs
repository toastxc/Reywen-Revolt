#[cfg(test)]
mod tests {
    use crate::structures::users::bot::{DataCreateBot, DataEditBot};
    use crate::tests::{tester_user, BOT, SERVER};

    #[tokio::test]
    async fn test_create_bot() {
        let client = tester_user();

        let data = DataCreateBot::new("womp");

        if let Err(error) = client.bot_create(&data).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_delete_bot() {
        let client = tester_user();

        let data = DataCreateBot::new("womp");
        match client.bot_create(&data).await {
            Ok(bot) => {
                if let Err(error) = client.bot_delete(&bot.id).await {
                    panic!("{:#?}", error);
                };
            }
            Err(e) => panic!("failed pretest (creating bot)\n{:#?}", e),
        }
    }

    #[tokio::test]
    async fn test_edit_bot() {
        let client = tester_user();

        let data_bot_create = DataCreateBot::new("wompywompy");
        let data_bot_edit = DataEditBot::new().set_name("cowdoyinthecity2");
        match client.bot_create(&data_bot_create).await {
            Ok(bot) => {
                if let Err(error) = client.bot_edit(&bot.id, &data_bot_edit).await {
                    panic!("{:#?}", error);
                };
            }
            Err(e) => panic!("failed pretest (creating bot)\n{:#?}", e),
        }
    }

    #[tokio::test]
    async fn test_bot_fetch() {
        let client = tester_user();

        if let Err(error) = client.bot_fetch(BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_owned() {
        let client = tester_user();

        if let Err(error) = client.bot_fetch_owned().await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_public() {
        let client = tester_user();

        if let Err(error) = client.bot_fetch_public(BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_invite() {
        let client = tester_user();

        if let Err(error) = client.bot_invite(BOT, SERVER, true).await {
            panic!("{:#?}", error);
        };
    }
}
