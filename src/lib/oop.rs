use crate::{structs::{auth::Auth, message::{RMessage, RMessagePayload, Masquerade, RReplies}}, lib::{rev_x::{rev_fetch_channel, rev_kick}, lreywen::send}};
use super::{rev_x::{rev_send, rev_del_2}, lreywen::reply_from};


#[derive(Debug, Clone, Default)]
pub struct Reywen {
    auth: Auth,
    input_message: RMessage,


}

impl Reywen {

    #[allow(dead_code)]
    pub fn new(auth: Auth, input_message: &RMessage) -> Self {
        let input_message = input_message.to_owned();

        Reywen
        {
            auth, input_message
        }
    }

    #[allow(dead_code)]
    pub async fn send(self, payload: RMessagePayload) -> Self {


        rev_send(&self.auth.token, &self.input_message.channel, payload).await;
        self
    }
    #[allow(dead_code)]
    pub async fn sender(self, content: &str) -> Self {
        let payload = RMessagePayload {
            content: Some(String::from(content)),
            attachments: None,
            replies: Some(vec![reply_from(&self.input_message)]),
            masquerade: None,
        };
        rev_send(&self.auth.token, &self.input_message.channel, payload).await;
        self
    }
    #[allow(dead_code)]
    pub async fn delete_msg(self, message_id: &str) -> Self {
        rev_del_2(&self.auth.token,&self.input_message.channel, message_id, ).await;
        self
    }

    #[allow(dead_code)]
    pub async fn member_kick(self, user: &str) -> Self {

        let server = rev_fetch_channel(&self.input_message.channel, &self.auth.token).await.unwrap().server;
        let payload = format!("**Kicking {}**", user);
        let user = user.replace(['@', '<', '>'], "");

        tokio::join!(
                rev_kick(&self.auth.token, &user, &server),
                send(&self.auth.token, &self.input_message, &payload),
                );
        self
    }
}

impl RMessagePayload {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn masquerade(mut self, masq: Masquerade) -> Self {
        self.masquerade = Some(masq);
        self

    }

    #[allow(dead_code)]
    pub fn reply_from(mut self, input: &RMessage) -> Self {
        self.replies = Some(vec![reply_from(input)]);
        self
    }

    #[allow(dead_code)]
    pub fn replies(mut self, replies: Vec<RReplies>) -> Self {
        self.replies = Some(replies);
        self
    }
}



impl Masquerade {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }
    #[allow(dead_code)]
    pub fn avatar(mut self, avatar: &str) -> Self {
        self.avatar = Some(String::from(avatar));
        self
    }
    #[allow(dead_code)]
    pub fn colour(mut self, colour: &str) -> Self {
        self.colour = Some(String::from(colour));
        self
    }
}