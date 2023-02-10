use iso8601_timestamp::Timestamp;

use crate::{
    methods::{
        bots::{
            self, BotResponse, DataCreateBot, DataEditBot, InviteBotDestination, OwnedBotsResponse,
            PublicBot,
        },
        channel,
        member::{self, DataBanCreate, DataMemberEdit},
        message::{self, DataEditMessage, OptionsMessageSearch},
        relationships::{self, DataSendFriendRequest, MutualResponse},
        server::{self, CreateServerResponse, DataCreateServer, DataEditServer},
        user::{self, DataEditUser},
    },
    structs::{
        auth::Auth,
        bots::Bot,
        message::{DataMessageSend, Masquerade, Message, SendableEmbed, SystemMessage},
        server::Server,
        user::User,
    },
};

/// DO is the builder pattern for safely and easily interacting with the Revolt API
/// While the API can be directly used it is not recommended
///
/// The fields auth and input_message are public but this is a temporary measure and will
/// become depricated!!
#[derive(Debug, Default, Clone)]
pub struct Do {
    pub auth: Auth,
    pub input_message: Message,
}

impl Do {
    pub fn new(auth: &Auth, input_message: &Message) -> Self {
        Do {
            auth: auth.to_owned(),
            input_message: input_message.to_owned(),
        }
    }

    pub fn bot(&self, bot_id: &str) -> BotMethod {
        BotMethod {
            auth: self.auth.clone(),
            bot_id: String::from(bot_id),
        }
    }

    pub fn channel(&self) -> ChannelMethod {
        ChannelMethod {
            auth: self.auth.clone(),
        }
    }
    pub async fn member(&self, server_id: &str) -> Option<MemberMethod> {
        let remember = MemberMethod {
            auth: self.auth.clone(),
            server: String::from(server_id),
        };

        server::fetch(&self.auth.domain, server_id, &self.auth.token)
            .await
            .map(|_| remember)
    }

    pub fn message(&self) -> MessageMethod {
        MessageMethod {
            auth: self.auth.clone(),
            input_message: self.input_message.clone(),
        }
    }
    pub fn server(&self) -> ServerMethod {
        ServerMethod {
            auth: self.auth.clone(),
            input_message: self.input_message.clone(),
        }
    }
    pub fn user(&self) -> UserMethod {
        UserMethod {
            auth: self.auth.clone(),
            input_message: self.input_message.clone(),
        }
    }

    pub fn auth(&self) -> AuthMethod {
        AuthMethod {
            auth: self.auth.clone(),
        }
    }
    pub fn input(&self) -> InputMessageMethod {
        InputMessageMethod {
            input_message: self.input_message.clone(),
        }
    }
}

pub struct BotMethod {
    auth: Auth,
    bot_id: String,
}

impl BotMethod {
    pub async fn create(&self, data: DataCreateBot) -> Option<Bot> {
        bots::create(&self.auth.domain, &self.auth.token, data).await
    }
    pub async fn fetch_public(&self) -> Option<PublicBot> {
        bots::fetch_public(&self.auth.domain, &self.auth.token, &self.bot_id).await
    }
    pub async fn invite(&self, data: InviteBotDestination) {
        bots::invite(&self.auth.domain, &self.auth.token, &self.bot_id, data).await
    }
    pub async fn fetch(&self) -> Option<BotResponse> {
        bots::fetch(&self.auth.domain, &self.auth.token, &self.bot_id).await
    }
    pub async fn delete(&self) {
        bots::delete(&self.auth.domain, &self.auth.token, &self.bot_id).await
    }
    pub async fn edit(&self, data: DataEditBot) -> Option<Bot> {
        bots::edit(&self.auth.domain, &self.auth.token, &self.bot_id, data).await
    }
    pub async fn owned(&self) -> Option<OwnedBotsResponse> {
        bots::owned(&self.auth.domain, &self.auth.token).await
    }
}

pub struct ChannelMethod {
    auth: Auth,
}

impl ChannelMethod {
    pub async fn delete(&self, channel_id: &str) {
        channel::delete(&self.auth.domain, channel_id, &self.auth.token).await
    }
    pub async fn edit(&self, channel_id: &str) {
        channel::edit(&self.auth.domain, channel_id, &self.auth.token).await
    }
    pub async fn fetch(&self, channel_id: &str) -> Option<crate::structs::channel::Channel> {
        channel::fetch(&self.auth.domain, channel_id, &self.auth.token).await
    }
}

pub struct MemberMethod {
    auth: Auth,
    server: String,
}

impl MemberMethod {
    pub async fn ban(&self, user_id: &str, reason: Option<&str>) {
        member::ban(
            &self.auth.domain,
            &self.server,
            &self.auth.token,
            user_id,
            DataBanCreate::new(reason),
        )
        .await
    }
    pub async fn edit(&self, user_id: &str, edit: DataMemberEdit) {
        member::edit(
            &self.auth.domain,
            &self.server,
            &self.auth.token,
            user_id,
            edit,
        )
        .await
    }
    pub async fn fetch(&self, user_id: &str) -> Option<crate::structs::server::Member> {
        member::fetch(&self.auth.domain, &self.server, &self.auth.token, user_id).await
    }
    pub async fn fetches(&self) -> Option<Vec<crate::structs::server::Member>> {
        member::fetches(&self.auth.domain, &self.server, &self.auth.token).await
    }
    pub async fn kick(&self, member: &str) {
        member::kick(&self.auth.domain, &self.server, &self.auth.token, member).await
    }
    pub async fn unban(&self, member: &str) {
        member::unban(&self.auth.domain, &self.server, &self.auth.token, member).await
    }
}

pub struct MessageMethod {
    auth: Auth,
    input_message: Message,
}

impl MessageMethod {
    pub async fn delete(&self, message: &str) {
        message::delete(
            &self.auth.domain,
            &self.input_message.channel,
            message,
            &self.auth.token,
        )
        .await
    }
    pub async fn edit(&self, message: &str, changes: DataEditMessage) {
        message::edit(
            &self.auth.domain,
            &self.input_message.channel,
            message,
            &self.auth.token,
            changes,
        )
        .await;
    }
    pub async fn fetch(&self) -> Option<Vec<Message>> {
        message::fetch(
            &self.auth.domain,
            &self.input_message.channel,
            &self.auth.token,
        )
        .await
    }
    pub async fn search(&self, search: OptionsMessageSearch) {
        message::search(
            &self.auth.domain,
            &self.input_message.channel,
            search,
            &self.auth.token,
        )
        .await
    }
    pub async fn sender(&self, message: &str) {
        message::send(
            &self.auth.domain,
            &self.input_message.channel,
            DataMessageSend::new().content(message),
            &self.auth.token,
        )
        .await;
    }

    pub async fn send(&self, message: DataMessageSend) {
        message::send(
            &self.auth.domain,
            &self.input_message.channel,
            message,
            &self.auth.token,
        )
        .await;
    }
}

pub struct RelationshipMethod {
    auth: Auth,
    user: String,
}

impl RelationshipMethod {
    pub async fn fetch_mutual_servers_and_friends(&self) -> Option<MutualResponse> {
        relationships::fetch_mutal_servers_and_friends(&self.auth.domain, &self.auth.token).await
    }
    pub async fn friend_accept(&self) -> Option<User> {
        relationships::accept_friend(&self.auth.domain, &self.auth.token, &self.user).await
    }
    pub async fn friend_deny(&self) -> Option<User> {
        relationships::deny_friend(&self.auth.domain, &self.auth.token, &self.user).await
    }
    pub async fn block(&self) -> Option<User> {
        relationships::block(&self.auth.domain, &self.auth.token, &self.user).await
    }
    pub async fn unblock(&self) -> Option<User> {
        relationships::unblock(&self.auth.domain, &self.auth.token, &self.user).await
    }
    pub async fn friend_request(&self, username: &str) -> Option<User> {
        let user = DataSendFriendRequest::new(username);
        relationships::friend_request(&self.auth.domain, &self.auth.token, user).await
    }
}

pub struct ServerMethod {
    auth: Auth,
    input_message: Message,
}

impl ServerMethod {
    pub async fn create(&self, payload: DataCreateServer) -> Option<CreateServerResponse> {
        server::create(&self.auth.domain, payload, &self.auth.token).await
    }
    pub async fn edit(&self, server_id: &str, payload: DataEditServer) {
        server::edit(&self.auth.domain, server_id, &self.auth.token, payload).await
    }

    pub async fn fetch(&self, server: &str) -> Option<Server> {
        crate::methods::server::fetch(&self.auth.domain, server, &self.auth.token).await
    }

    pub async fn leave(&self, server_id: &str) {
        crate::methods::server::leave(&self.auth.domain, server_id, &self.auth.token).await
    }

    pub async fn from_channel(&self) -> Option<String> {
        match channel::fetch(
            &self.auth.domain,
            &self.input_message.channel,
            &self.auth.token,
        )
        .await
        {
            Some(a) => a.server().1,
            None => None,
        }
    }
}

pub struct UserMethod {
    auth: Auth,
    input_message: Message,
}

impl UserMethod {
    pub async fn edit(&self, user: &str, edit: DataEditUser) {
        user::edit(&self.auth.domain, &self.auth.token, user, edit).await
    }
    pub async fn fetch(&self, user: &str) -> Option<User> {
        user::fetch(&self.auth.domain, &self.auth.token, user).await
    }
    pub async fn fetch_self(&self) -> Option<User> {
        user::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.input_message.author,
        )
        .await
    }
}

pub struct InputMessageMethod {
    input_message: Message,
}

impl InputMessageMethod {
    /// returns ID for Message
    pub fn id(&self) -> String {
        self.input_message.id.clone()
    }
    /// bool for id()
    pub fn id_is(&self, id: &str) -> bool {
        if id == self.input_message.id.as_str() {
            return true;
        };
        false
    }

    /// returns channel for Message
    pub fn channel(&self) -> String {
        self.input_message.channel.clone()
    }
    /// bool for channel()
    pub fn channel_is(&self, id: &str) -> bool {
        if id == self.input_message.channel.as_str() {
            return true;
        };
        false
    }
    /// returns channel for Message
    pub fn author(&self) -> String {
        self.input_message.author.clone()
    }
    /// bool for channel()
    pub fn author_is(&self, id: &str) -> bool {
        if id == self.input_message.author.as_str() {
            return true;
        };
        false
    }

    /// returns content for Message, if none return an empty string
    pub fn content(&self) -> String {
        self.input_message.content.clone().unwrap_or_default()
    }
    /// bool for content()
    pub fn content_is(&self, content: &str) -> bool {
        if let Some(self_content) = &self.input_message.content {
            return self_content.as_str() == content;
        };
        false
    }

    /// returns SystemMessage
    pub fn system_message(&self) -> Option<SystemMessage> {
        self.input_message.system.clone()
    }

    /// returns if the message has been edited
    pub fn edited(&self) -> Option<Timestamp> {
        self.input_message.edited
    }

    /// returns if the message has been edited
    pub fn mentions(&self) -> Option<Vec<String>> {
        self.input_message.mentions.clone()
    }

    /// returns if the message has been edited
    pub fn replies(&self) -> Option<Vec<String>> {
        self.input_message.replies.clone()
    }

    /// returns if the message has been edited
    pub fn embeds(&self) -> Option<Vec<SendableEmbed>> {
        self.input_message.embeds.clone()
    }

    /// returns if the message has been edited
    pub fn masquerade(&self) -> Option<Masquerade> {
        self.input_message.masquerade.clone()
    }
    /// vector of content
    pub fn convec(&self) -> Vec<String> {
        vecify(&self.input_message.content.clone().unwrap_or_default())
    }
}

pub struct AuthMethod {
    auth: Auth,
}

impl AuthMethod {
    pub fn sudoers(&self) -> Vec<String> {
        self.auth.sudoers.clone()
    }
    pub fn is_sudoer(&self, user: &str) -> bool {
        self.auth.sudoers.contains(&String::from(user))
    }
}

/// simple fn for converting &str to Vec<String> (not Vec<&str>)
fn vecify(input: &str) -> Vec<String> {
    let mut master: Vec<String> = Vec::new();
    for x in input.split(' ') {
        master.push(x.to_string())
    }

    master
}

pub struct Web {}
impl Web {
    pub fn error(e: reqwest::Error, message: &str) {
        println!("HTTP ERROR: {e}\n{message}")
    }
}
