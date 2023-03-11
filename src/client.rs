use iso8601_timestamp::Timestamp;
use log::warn;

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

    pub fn channel(&self, channel: Option<&str>) -> ChannelMethod {
        let channel = match channel {
            Some(a) => String::from(a),
            None => self.input_message.channel.clone(),
        };

        ChannelMethod {
            auth: self.auth.clone(),
            channel,
        }
    }
    pub async fn member(&self, server_id: Option<&str>, member: Option<&str>) -> MemberMethod {
        let member = member.unwrap_or(&self.input_message.author);

        let server = match server_id {
            Some(a) => String::from(a),
            None => channel::fetch(
                &self.auth.domain,
                &self.auth.token,
                &self.auth.header,
                &self.input_message.channel,
            )
            .await
            .unwrap()
            .server()
            .1.unwrap()

        };

        MemberMethod {
            auth: self.auth.clone(),
            server: String::from(server),
            member: String::from(member),
        }
    }

    pub fn message(&self) -> MessageMethod {
        MessageMethod {
            auth: self.auth.clone(),
            input_message: self.input_message.clone(),
        }
    }
    pub fn relationship(&self, user: &str) -> RelationshipMethod {
        RelationshipMethod {
            auth: self.auth.clone(),
            user: String::from(user),
        }
    }
    pub async fn server(&self, server_id: Option<&str>) -> ServerMethod {
        let server = match server_id {
            Some(a) => String::from(a),
            None => channel::fetch(
                &self.auth.domain,
                &self.auth.token,
                &self.auth.header,
                &self.input_message.channel,
            )
            .await
            .unwrap()
            .server()
            .1
            .unwrap(),
        };

        ServerMethod {
            auth: self.auth.clone(),
            server,
            input_message: self.input_message.clone(),
        }
    }
    pub fn user(&self, user_id: Option<&str>) -> UserMethod {
        let user = user_id.unwrap_or(&self.input_message.author);

        UserMethod {
            auth: self.auth.clone(),
            input_message: self.input_message.clone(),
            user: String::from(user),
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
        bots::create(&self.auth.domain, &self.auth.token, &self.auth.header, data).await
    }
    pub async fn fetch_public(&self) -> Option<PublicBot> {
        bots::fetch_public(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.bot_id,
        )
        .await
    }
    pub async fn invite(&self, data: InviteBotDestination) {
        bots::invite(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.bot_id,
            data,
        )
        .await
    }
    pub async fn fetch(&self) -> Option<BotResponse> {
        bots::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.bot_id,
        )
        .await
    }
    pub async fn delete(&self) {
        bots::delete(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.bot_id,
        )
        .await
    }
    pub async fn edit(&self, data: DataEditBot) -> Option<Bot> {
        bots::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.bot_id,
            data,
        )
        .await
    }
    pub async fn owned(&self) -> Option<OwnedBotsResponse> {
        bots::owned(&self.auth.domain, &self.auth.token, &self.auth.header).await
    }
}

pub struct ChannelMethod {
    auth: Auth,
    channel: String,
}

impl ChannelMethod {
    pub async fn delete(&self) {
        channel::delete(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.channel,
        )
        .await
    }
    pub async fn edit(&self) {
        channel::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.channel,
        )
        .await
    }
    pub async fn fetch(&self) -> Option<crate::structs::channel::Channel> {
        channel::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.channel,
        )
        .await
    }
}

pub struct MemberMethod {
    auth: Auth,
    server: String,
    member: String,
}

impl MemberMethod {
    pub async fn ban(&self, reason: Option<&str>) {
        member::ban(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            &self.member,
            DataBanCreate::new(reason),
        )
        .await
    }
    pub async fn edit(&self, edit: DataMemberEdit) {
        member::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            &self.member,
            edit,
        )
        .await
    }
    pub async fn fetch(&self) -> Option<crate::structs::server::Member> {
        member::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            &self.member,
        )
        .await
    }
    pub async fn fetch_all(&self) -> Option<Vec<crate::structs::server::Member>> {
        member::fetch_all(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
        )
        .await
    }
    pub async fn kick(&self) {
        member::kick(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            &self.member,
        )
        .await
    }
    pub async fn unban(&self) {
        member::unban(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            &self.member,
        )
        .await
    }
}

pub struct MessageMethod {
    auth: Auth,
    input_message: Message,
}

impl MessageMethod {
    pub async fn delete(&self, message_id: Option<&str>) {
        let message = match message_id {
            Some(a) => a,
            None => &self.input_message.id,
        };

        message::delete(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
            message,
        )
        .await
    }
    pub async fn edit(&self, message: &str, changes: DataEditMessage) {
        message::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
            message,
            changes,
        )
        .await;
    }
    pub async fn fetch(&self) -> Option<Vec<Message>> {
        message::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
        )
        .await
    }
    pub async fn search(&self, search: OptionsMessageSearch) {
        message::search(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
            search,
        )
        .await
    }
    pub async fn sender(&self, message: &str) {
        message::send(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
            DataMessageSend::new().content(message),
        )
        .await;
    }

    pub async fn send(&self, message: DataMessageSend) {
        message::send(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.input_message.channel,
            message,
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
        relationships::fetch_mutal_servers_and_friends(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
        )
        .await
    }
    pub async fn friend_accept(&self) -> Option<User> {
        relationships::accept_friend(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
        )
        .await
    }
    pub async fn friend_deny(&self) -> Option<User> {
        relationships::deny_friend(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
        )
        .await
    }
    pub async fn block(&self) -> Option<User> {
        relationships::block(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
        )
        .await
    }
    pub async fn unblock(&self) -> Option<User> {
        relationships::unblock(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
        )
        .await
    }
    pub async fn friend_request(&self, username: &str) -> Option<User> {
        let user = DataSendFriendRequest::new(username);
        relationships::friend_request(&self.auth.domain, &self.auth.token, &self.auth.header, user)
            .await
    }
}

pub struct ServerMethod {
    auth: Auth,
    server: String,
    input_message: Message,
}

impl ServerMethod {
    pub fn member(&self, member: Option<&str>) -> MemberMethod {
        let member = match member {
            Some(a) => String::from(a),
            None => self.input_message.author.clone(),
        };

        MemberMethod {
            auth: self.auth.clone(),
            server: self.server.clone(),
            member,
        }
    }

    pub async fn create(&self, data: DataCreateServer) -> Option<CreateServerResponse> {
        server::create(&self.auth.domain, &self.auth.token, &self.auth.header, data).await
    }
    pub async fn edit(&self, data: DataEditServer) {
        server::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
            data,
        )
        .await
    }

    pub async fn fetch(&self) -> Option<Server> {
        crate::methods::server::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
        )
        .await
    }

    pub async fn leave(&self) {
        crate::methods::server::leave(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.server,
        )
        .await
    }
}

pub struct UserMethod {
    auth: Auth,
    input_message: Message,
    user: String,
}

impl UserMethod {
    pub async fn edit(&self, edit: DataEditUser) {
        user::edit(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
            edit,
        )
        .await
    }
    pub async fn fetch(&self) -> Option<User> {
        user::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
            &self.user,
        )
        .await
    }
    pub async fn fetch_self(&self) -> Option<User> {
        user::fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.auth.header,
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

    /// if input message contains a string
    pub fn contains(&self, content: &str) -> bool {
        let str = vecify(content);
        str.contains(&String::from(content))
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
        let res = format!("{}\n{message}", e.status().unwrap_or_default());
        println!("{res}");
        warn!("{res}");
    }
}
