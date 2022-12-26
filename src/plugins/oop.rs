  // a sandbox for experimenting with OOP design patterns for abstraction and DX
    // lets create a simple hello world thingo

use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::rev_send};
use crate::send;
#[derive(Debug)]
enum ReywenSys{

    Send{content: &'static str},
    SendMasq{content: &'static str, masq: Masquerade},
    Delete,
    
}

impl ReywenSys {

    async fn run(&self, auth: Auth, message_input: RMessage) {
        match self {
            Self::Send{content: c } => {
                send(&auth.token, &message_input, c ).await
                },
            Self::SendMasq { content, masq } => {}
            Self::Delete => {}
            
        }
    }

}

pub async fn oop_main(input_message: RMessage) {

   
    // send message
    ReywenSys::Send { content: "" };
    
    // send message with masq
    let masq_payload = Masquerade{
        colour: None,
        avatar: None,
        name: Some(String::from("greer")),
    };

    ReywenSys::SendMasq{content: "greer", masq: {masq_payload}};
    
    
    
  

    // example usage for bot
    let content = input_message.content.unwrap();
    
    match &content as &str {
       "greer" => {
           ReywenSys::Send{ content: ("balls") };
       },
       _ => {},

    }    
    
    
}
