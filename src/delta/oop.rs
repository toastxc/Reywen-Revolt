use crate::{
    delta::delta::{rev_fetch_channel, rev_kick},
    quark::delta::{
        auth::Auth,
        message::{Masquerade, RMessage, RMessagePayload, RReplies},
        user::RUserFetch,
    },
};

use super::{
    delta::{rev_del_2, rev_send, rev_user},
    lreywen::reply_from,
};

#[derive(Debug, Clone, Default)]
pub struct Reywen {
    pub auth: Auth,
    pub input_message: RMessage,
}

impl Reywen {
    #[allow(dead_code)]
    pub fn new(auth: Auth, input_message: &RMessage) -> Self {
        let input_message = input_message.to_owned();

        Reywen {
            auth,
            input_message,
        }
    }

    #[allow(dead_code)]
    pub async fn send(self, payload: RMessagePayload) -> Self {
        rev_send(
            &self.auth.domain,
            &self.input_message.channel,
            &self.auth.token,
            payload,
        )
        .await;
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
        rev_send(
            &self.auth.domain,
            &self.input_message.channel,
            &self.auth.token,
            payload,
        )
        .await;
        self
    }
    #[allow(dead_code)]
    pub async fn delete_msg(self, message_id: &str) -> Self {
        rev_del_2(
            &self.auth.domain,
            &self.input_message.channel,
            message_id,
            &self.auth.token,
        )
        .await;
        self
    }

    #[allow(dead_code)]
    pub async fn member_kick(self, user: &str) -> Self {
        let server = rev_fetch_channel(
            &self.auth.domain,
            &self.input_message.channel,
            &self.auth.token,
        )
        .await
        .unwrap()
        .server;
        let content = format!("**Kicking {}**", user);
        let user = user.replace(['@', '<', '>'], "");

        let payload = RMessagePayload::new()
            .content(&content)
            .reply_from(&self.input_message);

        tokio::join!(
            rev_kick(&self.auth.domain, &server, &user, &self.auth.token),
            //send(&self.auth.token, &self.input_message, &payload),
            rev_send(
                &self.auth.domain,
                &self.input_message.channel,
                &self.auth.token,
                payload
            ),
        );
        self
    }
    #[allow(dead_code)]
    pub async fn get_user(self, user: &str) -> Option<RUserFetch> {
        let user = rev_user(&self.auth.domain, user, &self.auth.token).await;
        user
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
