

use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload, RReplies}}, rev_x::{rev_send, rev_del_2}};
use crate::plugins::lreywen::reply_from;
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
    
        
        rev_send(&self.auth.token, channel, payload).await;
        self
    }
    pub async fn sender(self, content: &str, input_message: &RMessage) -> Self {
        
        let payload = RMessagePayload {
            content: Some(String::from(content)),
            attachments: None,
            replies: Some(vec![reply_from(input_message)]),
            masquerade: None,
        };
        rev_send(&self.auth.token, &input_message.channel, payload).await;
        self
    }
    pub async fn delete_msg(self, message_id: &str, input_message: &RMessage, ) -> Self {
        rev_del_2(&self.auth.token, message_id, &input_message.channel).await;
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
        self.replies = Some(vec![reply_from(input)]);
        self
    }
    
    pub fn replies(mut self, replies: Vec<RReplies>) -> Self {
        self.replies = Some(replies);
        self
    }
}




pub async fn oop_main(auth: Auth, input_message: &RMessage) {
    // stops function from running
    return;
    
    // new client based on stored auth keys
    let client = Reywen::new(auth);
  
    // clients can be constructed from hard coded values like so (commented for warning)
    //let other_client = Reywen::new(Auth {token: String::from("TOKEN"), bot_id: String::from("ID"), sudoers: Vec::new()});
  
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
        .reply(input_message)
        // replies allows manual control values for replies
        .replies(replies);
        
   
    //this is a simple example of message logic
    // if the content of the previous message from websocket == "?tester"
    // send payload as defined ^
    if input_message.content == Some(String::from("?tester")) {
        client.clone().send(payload, &input_message.channel).await;
    };
    
    // another method, sender is even simpler but grants less control over options
    client.clone().sender("owo", input_message).await;
    
    
    // usually bots have many commands and operations, and for this
    // a match is much more practical
    // another thing we'll do  is split the input message into a vector
    // seperated by spaces, unfortunately handling vectors the wrong way
    // can lead to crashes so we will handle that too
    
    // this closes the function if there is no content
    let input = match input_message.clone().content {
        None => return,
        Some(a) => a,
    };
    
    // cconvec short for content vector (self explanatory)
    let convec: Vec<&str> = input.split(' ').collect();
    if convec.is_empty() {return};
    
    // matches make code a lot cleaner for multiple arguements
    let sender_content = match convec[0] {
        
        // if input == ?mog, define sender as AMOGUS
        "?mog" => "AMOGUS",
        "i dont like reywen" => "meanie ;w;",
        
        // if no conditions are met, return an empty string
        _ => return
    };
    
    client.sender(sender_content, input_message).await;
    
    
    // the message input is a bad word, remove it
    if convec.contains(&"badword") {
        client.delete_msg(&input_message._id, input_message).await;  
    };
    
    // a more robust solution for banned words
    let wordlist = vec!["?word1", "?word1", "?word1", "?word1", "?word1"];
    
    for x in wordlist.iter() {
        if convec.contains(x) {
            client.delete_msg(&input_message._id, input_message).await;  
        }
    };
    
    
    
    
     
    
    
}