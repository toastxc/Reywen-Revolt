use crate::{
    methods::{message, user},
    structs::{
        auth::Auth,
        message::{DataMessageSend, Message},
        server::Server,
        user::User,
    },
};

/// DO is the builder pattern for safely and easily interacting with the Revolt API
/// While the API can be directly used it is not recommended
///
/// Both fields Auth and Message are private, but there are methods extracting these
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

    /// returns text content for Message, if none return an empty string
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

    /// vector of content
    pub fn convec(&self) -> Vec<String> {
        vecify(&self.input_message.content.clone().unwrap_or_default())
    }

    /// returns the author for Message
    pub fn author(&self) -> String {
        self.input_message.author.clone()
    }

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

    /// fetches User based on the input_message author - None for failure
    pub async fn self_fetch(&self) -> Option<User> {
        crate::methods::user::user_fetch(
            &self.auth.domain,
            &self.auth.token,
            &self.input_message.author,
        )
        .await
    }

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

    /// fetches server details - None for failure
    pub async fn fetch_server(&self, server: &str) -> Option<Server> {
        crate::methods::server::server_fetch(&self.auth.domain, server, &self.auth.token).await
    }
}

/// simple abstracton for converting &str to Vec<String> (not Vec<&str>)
fn vecify(input: &str) -> Vec<String> {
    let mut master: Vec<String> = Vec::new();
    for x in input.split(' ') {
        master.push(x.to_string())
    }

    master
}
