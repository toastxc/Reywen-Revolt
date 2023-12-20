#[cfg(test)]
mod tests {
    use crate::structures::emoji::{DataCreateEmoji, EmojiParent};
    use crate::tests::tester_user;

    #[tokio::test]
    async fn emoji_test_all() {
        let client = tester_user();

        // create emoji

        // let dat = DataCreateEmoji {
        //     name: "hello".to_string(),
        //     parent: EmojiParent::Server {
        //         id: String::from("01H321YNJZZMF1SYPEY4S9B0R0"),
        //     },
        //     nsfw: false,
        // };

        let emoji = client
            .emoji_create(
                &DataCreateEmoji::new("aaaaaaaaaaaa", "01H321YNJZZMF1SYPEY4S9B0R0"),
                "01HHH9XGBG2PWSED5X36JSJ8K7",
            )
            .await
            .unwrap();

        //
        // println!("{:?}", emoji);
        // fetch emoji
        // client.emoji_fetch(&emoji.id).await.unwrap();
        // delete emoji
        // client.emoji_delete(&emoji.id).await.unwrap();

        client
            .emoji_fetch("01HHH9XGBG2PWSED5X36JSJ9K7")
            .await
            .unwrap();
    }
}
