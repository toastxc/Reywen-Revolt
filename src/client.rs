use iso8601_timestamp::Timestamp;

use crate::{
    methods::{message, user},
    structs::{
        auth::Auth,
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
    /// Creates a Do from authenicaton details and the input message
    pub fn new(auth: &Auth, input_message: &Message) -> Self {
        Do {
            auth: auth.to_owned(),
            input_message: input_message.to_owned(),
        }
    }
    // ########################### INPUT_MESSAGE ###############################

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

    /// checks if bot is reading its own message
    pub fn author_is_bot(&self) -> bool {
        self.auth.bot_id == self.input_message.author
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

    // ########################### AUTH ###############################

    pub fn sudoers(&self) -> Vec<String> {
        self.auth.sudoers.clone()
    }
    pub fn is_sudoer(&self, user: &str) -> bool {
        self.auth.sudoers.contains(&String::from(user))
    }

    /// vector of content
    pub fn convec(&self) -> Vec<String> {
        vecify(&self.input_message.content.clone().unwrap_or_default())
    }

    // ########################### API - POST ###############################
    /// sends a message, all fields are ignored besides content for Message
    pub async fn sender(&self, message: &str) {
        message::message_send(
            &self.auth.domain,
            &self.input_message.channel,
            DataMessageSend::new().content(message),
            &self.auth.token,
        )
        .await;
    }
    /// sends a message - requires the Message data structure
    pub async fn send(&self, message: DataMessageSend) {
        message::message_send(
            &self.auth.domain,
            &self.input_message.channel,
            message,
            &self.auth.token,
        )
        .await;
    }

    // ########################### API - GET ###############################
    /// fetches User based on the input_message author - None for failure
    pub async fn self_fetch(&self) -> Option<User> {
        crate::methods::user::user_fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.input_message.author,
        )
        .await
    }

    /// fetches server details - None for failure
    pub async fn fetch_server(&self, server: &str) -> Option<Server> {
        crate::methods::server::server_fetch(&self.auth.domain, server, &self.auth.token).await
    }

    // ########################### API - PATCH ###############################

    /// edits user based on input_message author
    pub async fn edit_self(&self, edit: user::DataEditUser) {
        crate::methods::user::user_edit(
            &self.auth.domain,
            &self.auth.token,
            &self.input_message.author,
            edit,
        )
        .await
    }
}
// ########################### OTHER ###############################

/// simple fn for converting &str to Vec<String> (not Vec<&str>)
fn vecify(input: &str) -> Vec<String> {
    let mut master: Vec<String> = Vec::new();
    for x in input.split(' ') {
        master.push(x.to_string())
    }

    master
}
