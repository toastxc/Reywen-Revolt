#[cfg(test)]
mod tests {
    use crate::{
        client::methods::{
            channel::DataEditChannel,
            group::DataCreateGroup,
            message::{
                DataBulkDelete, DataEditMessage, DataMessageSearch, DataMessageSend,
                DataQueryMessages, DataUnreact,
            },
        },
        structures::permissions::{calculator::Permissions, definitions::Permission},
        tests::{tester_bot, tester_user, CHANNEL, GROUP},
    };

    #[tokio::test]
    async fn test_delete_channel() {
        let client = tester_bot();

        if let Err(error) = client.channel_delete(&"01GXDMSSJTXB14EA7J4R77B778").await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_channel_edit() {
        let client = tester_bot();

        let data = DataEditChannel::new()
            .set_name("benis")
            .set_description("wenis");

        if let Err(error) = client.channel_edit(&CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_channel_fetch() {
        let client = tester_bot();

        if let Err(error) = client.channel_fetch("01H321YNJZXSJFJ8TKHZ1P5SGX").await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn group_add_member() {
        let client = tester_user();

        if let Err(error) = client
            .group_member_add(&GROUP, &"01FQ6SDAVSV5X1B4A7JXNB4FZV")
            .await
        {
            panic!("{:#?}", error);
        }
    }

    // todo does not work
    #[tokio::test]
    async fn test_create_group() {
        let client = tester_user();

        let data = DataCreateGroup::new("womp")
            .add_user("01FQ6SDAVSV5X1B4A7JXNB4FZV")
            .set_nsfw(false);

        println!("{:#?}", data);
        if let Err(error) = client.group_create(&data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_group_remove_member() {
        let client = tester_user();

        if let Err(error) = client
            .group_member_remove("01GYM0JBNKWRJYX56F9GYABS4R", "01FQ6SDAVSV5X1B4A7JXNB4FZV")
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_invite_create() {
        let client = tester_user();

        if let Err(error) = client.channel_invite_create(&CHANNEL).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_group_member_fetch() {
        let client = tester_user();

        if let Err(error) = client.group_member_fetch_all(&GROUP).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_ack() {
        let client = tester_user();

        if let Err(error) = client
            .message_ack(&CHANNEL, "01GYMBW4XV6TF3199RVFXWWVQ7")
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_bulk_delete() {
        let client = tester_bot();

        let data = DataBulkDelete::new().add_message("01GYMCHDB9Q1ETS4KP9NG1WW32");
        if let Err(error) = client.message_bulk_delete(&CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_clear_reactions() {
        let client = tester_bot();

        if let Err(error) = client
            .message_reaction_remove_all(&CHANNEL, "01GYMCHDB9WRYN8WEVG25FESVS")
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_delete() {
        let client = tester_user();

        let create_result_data = DataMessageSend::new().set_content("reywen_test");
        let create_result = client.message_send(&CHANNEL, &create_result_data).await;

        if let Err(error) = create_result {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let cr_data = create_result.ok().unwrap();
        if let Err(error) = client.message_delete(&CHANNEL, cr_data.id.as_str()).await {
            panic!("delete message failed {:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_edit() {
        let client = tester_bot();

        let original_message_data = DataMessageSend::new().set_content("original content");
        let original_message = client.message_send(&CHANNEL, &original_message_data).await;

        if let Err(error) = original_message {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let original_message_success = original_message.ok().unwrap();
        let edit_message_data = DataEditMessage::new().set_content("edited content");
        if let Err(error) = client
            .message_edit(
                &CHANNEL,
                original_message_success.id.as_str(),
                &edit_message_data,
            )
            .await
        {
            panic!("edit message failed {:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_fetch() {
        let client = tester_user();
        let original_message_data = DataMessageSend::new().set_content("fetch test");
        let original_message = client.message_send(&CHANNEL, &original_message_data).await;

        if let Err(error) = original_message {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let og_succ = original_message.ok().unwrap();
        if let Err(error) = client.message_fetch(&CHANNEL, og_succ.id.as_str()).await {
            panic!("fetch message failed {:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_query() {
        let client = tester_user();

        let data = DataQueryMessages::new()
            .set_limit(24)
            .set_include_users(true);
        if let Err(error) = client.message_query(&CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_emoji_react() {
        let client = tester_user();

        if let Err(error) = client
            .message_reaction_add(
                CHANNEL,
                "01GYP3PB8VBS6B6DE9YVRK8C69",
                "01G83M8KJE4KGQCQT2PP5EH3VT",
            )
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_search() {
        let client = tester_user();

        let data = DataMessageSearch::new("womp").set_include_users(true);
        if let Err(error) = client.message_search(&CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_send() {
        let client = tester_user();

        let data = DataMessageSend::new().set_content("womo");
        if let Err(error) = client.message_send(&CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_unreact() {
        let client = tester_user();

        let data = DataUnreact::new();
        if let Err(error) = client
            .message_reaction_remove(
                &CHANNEL,
                "01GYP6KM9C51XFQNG13ANR4PT1",
                "01GQE86CT9MKAHPTG55HMTG7TR",
                &data,
            )
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_permission_set() {
        let client = tester_bot();
        let data = Permissions::default().add_allow(Permission::SendMessage);

        if let Err(error) = client
            .channel_permissions_set(
                "01H3226059WB9HTRBR3YZ9M6Q9",
                "01H4T0RZ1YD2Q7G7ZTAAAXBKH7",
                &data.export(),
            )
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_permissions_set_default() {
        let client = tester_user();

        let data = Permissions::default()
            .add_allow(Permission::ViewChannel)
            .add_allow(Permission::SendMessage)
            .export();

        println!("{:#?}", data);

        if let Err(error) = client
            .channel_permissions_set_default("01GXDKYV0P4T6DHNNG7M15CQ5R", &data)
            .await
        {
            panic!("{:#?}", error);
        }
    }
}
