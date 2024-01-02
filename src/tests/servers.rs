#[cfg(test)]
mod tests {
    use crate::structures::permissions::calculator::Permissions;
    use crate::structures::permissions::definitions::Permission;
    use crate::structures::permissions::DataEditRole;
    use crate::structures::permissions::DataRoleCreate;
    use crate::structures::server::member::DataMemberEdit;
    use crate::structures::server::DataChannelCreate;
    use crate::structures::server::DataCreateServer;
    use crate::structures::server::DataEditServer;
    use crate::tests::tester_bot;
    use crate::tests::tester_user;
    use crate::tests::ROLE;
    use crate::tests::SERVER;
    use crate::tests::USER;

    #[tokio::test]
    async fn test_member_edit() {
        let client = tester_bot();

        let data = DataMemberEdit::default();

        if let Err(curl) = client.member_edit(SERVER, USER, &data).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_remove() {
        let client = tester_bot();

        if let Err(curl) = client.member_remove(SERVER, USER).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_fetch() {
        let client = tester_bot();

        if let Err(curl) = client
            .member_fetch("01H321YNJZZMF1SYPEY4S9B0R0", "01FSRTTGJC1XJ6ZEQJMSX8Q96C")
            .await
        {
            panic!("{:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_member_fetch_all() {
        let client = tester_bot();

        if let Err(curl) = client.member_fetch_all(SERVER).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_mkrm() {
        let client = tester_bot();

        // ban user

        if let Err(curl) = client.ban_create(SERVER, USER, None).await {
            panic!("ban user {:#?}", curl);
        }

        if let Err(curl) = client.ban_remove(SERVER, USER).await {
            panic!("remove banned user {:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_list() {
        let client = tester_bot();

        if let Err(curl) = client.ban_list(SERVER).await {
            panic!("list banned users {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let client = tester_bot();

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = client.channel_create(SERVER, &create_chan).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_permission_set_default() {
        let client = tester_bot();
        let data = Permissions::default()
            .add_allow(Permission::ViewChannel)
            .add_allow(Permission::KickMembers);

        if let Err(error) = client.server_permission_set_default(SERVER, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_fetch_invites() {
        let client = tester_bot();

        if let Err(error) = client.invites_fetch(SERVER).await {
            panic!("{:#?}", error);
        }
    }
    #[tokio::test]
    async fn test_permission_set() {
        let client = tester_bot();

        let perms = Permissions::default()
            .add_allow(Permission::ViewChannel)
            .add_allow(Permission::KickMembers);

        if let Err(error) = client.server_permission_set(SERVER, ROLE, &perms).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_create() {
        let client = tester_bot();

        let data = DataRoleCreate::new("dummyrole");

        if let Err(error) = client.roles_create(SERVER, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_delete() {
        let client = tester_bot();

        if let Err(error) = client
            .roles_delete(SERVER, "01GXG24BV8QMSFWXKFRHZV30AY")
            .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_roles_edit() {
        let client = tester_bot();

        let data = DataEditRole::new();

        if let Err(error) = client.roles_edit(SERVER, ROLE, &data).await {
            panic!("{:#?}", error);
        }
    }

    // todo unresolved issue
    #[tokio::test]
    async fn test_server_create() {
        let client = tester_bot();

        let data = DataCreateServer::new("DummyServer");

        if let Err(error) = client.server_create(&data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_edit() {
        let client = tester_bot();
        let data = DataEditServer::new();

        if let Err(error) = client.server_edit(SERVER, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_fetch() {
        let client = tester_bot();

        if let Err(error) = client.server_fetch(SERVER).await {
            panic!("{:#?}", error);
        }
    }

    // todo untested
    #[tokio::test]
    async fn test_server_ack() {
        let client = tester_user();

        if let Err(error) = client.server_ack(SERVER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_delete() {
        let client = tester_bot();

        if let Err(error) = client.server_delete(SERVER).await {
            panic!("{:#?}", error);
        }
    }
}
