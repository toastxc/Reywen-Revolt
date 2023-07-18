#[cfg(test)]
mod tests {
    use crate::{
        client::methods::user::DataEditUser,
        tests::{tester_bot, tester_user, USER, USER_NOTSELF},
    };

    #[tokio::test]
    async fn test_user_edit() {
        let client = tester_user();

        if let Err(error) = client.user_edit(&USER, &DataEditUser::new()).await {
            panic!("{:#?}", error);
        }
    }
    #[tokio::test]
    async fn test_user_fetch() {
        let client = tester_bot();

        if let Err(error) = client.user_fetch(&USER).await {
            panic!("{:#?}", error);
        }
    }
    #[tokio::test]
    async fn test_fetch_mutual() {
        let client = tester_user();

        if let Err(error) = client.fetch_mutual(USER_NOTSELF).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_user_fetch_self() {
        let client = tester_user();

        if let Err(error) = client.user_fetch_self().await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_user_block_remove() {
        let client = tester_user();

        if let Err(error) = client.user_block_remove(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_dm_open() {
        let client = tester_user();

        if let Err(error) = client.dm_open(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_dm_fetch_all() {
        let client = tester_user();

        if let Err(error) = client.dm_fetch_all().await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_default_avatar_fetch() {
        let client = tester_user();

        if let Err(error) = client.default_avatar_fetch(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_user_flags_fetch() {
        let client = tester_user();

        if let Err(error) = client.user_flags_fetch(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_user_block() {
        let client = tester_user();

        if let Err(error) = client.user_block(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_friend_request_send() {
        let client = tester_user();

        if let Err(error) = client.friend_request_send(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_friend_request_accept() {
        let client = tester_user();

        if let Err(error) = client.friend_request_accept(&USER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_friend_request_reject_remove() {
        let client = tester_user();

        if let Err(error) = client.friend_request_reject(&USER).await {
            panic!("{:#?}", error);
        }
    }
}
