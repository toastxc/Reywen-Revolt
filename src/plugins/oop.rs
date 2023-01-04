use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload, RReplies}}, rev_x::rev_send};

#[derive(Debug, Clone, Default)]
pub struct Reywen {
  auth: Auth
}

impl Reywen {
    
    pub fn new(auth: Auth) -> Self {
        Reywen
        {
            auth
        }
    }
    
  
    pub async fn send(self, payload: RMessagePayload, channel: &str) -> Self {
        //self.send = payload;
        
        rev_send(&self.auth.token, channel, payload).await;
        self
    }
}

impl RMessagePayload {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: &'static str) -> Self {
        self.content = Some(content.to_string());
        self
    }
    
    pub fn masquerade(mut self, masq: Masquerade) -> Self {
        self.masquerade = Some(masq);
        self
        
    }
    
    pub fn reply(mut self, input: &RMessage) -> Self {
        self.replies = Some(vec![reply_from(&input)]);
        self
    }
    
    pub fn replies(mut self, replies: Vec<RReplies>) -> Self {
        self.replies = Some(replies);
        self
    }
}




pub async fn oop_main(auth: Auth, input_message: RMessage) {
    
    // new client based on stored auth keys
    let client = Reywen::new(auth);
  
    // clients can be constructed from hard coded values like so
    let other_client = Reywen::new(Auth {token: String::from("TOKEN"), bot_id: String::from("ID"), sudoers: Vec::new()});
  
    // Masquerade is a feature in revolt for sending messages with a 
    // different profile, requires permissions from a server
    let masq = Masquerade { name: Some(String::from("Greg")), avatar: None, colour: None };
    
  
   // replies payload
   let replies = vec![
       RReplies {
           id: input_message._id.to_owned(),
           mention: false,
       }  
   ];
   
    
    
    
    // RMessagePayload is the type for sending messages to Revolt API
    // all fields are optional
    let payload  = RMessagePayload::new()
        // message content
        .content("Hello i am greg!")
        .masquerade(masq)
        // reply from constructs a reply payload based on an input message
        .reply(&input_message)
        // replies allows manual control values for replies
        .replies(replies);
        
   
    //this is a simple example of message logic
    // if the content of the previous message from websocket == "?tester"
    // send payload as defined ^
    if input_message.content == Some("?tester".to_string()) {
        client.send(payload, &input_message.channel).await;
    }
    
}