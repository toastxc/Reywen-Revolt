  // a sandbox for experimenting with OOP design patterns for abstraction and DX
    // lets create a simple hello world thingo

use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::rev_send};
use crate::send;

#[derive(Debug)]
enum ReywenSys{

    Send{content: &'static str},
    SendMasq{content: &'static str, masq: Masquerade},
    Delete,
    None,
    
}
pub async fn oop_main(auth: Auth, input_message: RMessage ) {
    
impl ReywenSys {

    async fn run(&self, auth: Auth, message_input: RMessage) {
        match self {
            Self::Send{content: c } => {
                send(&auth.token, &message_input, c ).await
            },
            
            
            Self::SendMasq { content, masq } => {
      
                let payload = RMessagePayload {
                    content: Some(content.to_string()),
                    attachments: None,
                    replies: None,
                    masquerade: Some(masq.to_owned()),   
                    
                };
                rev_send(&auth.token, &message_input.channel, payload).await          
            }
            Self::Delete => {},
            
            Self::None => {},
            
        };
    }

}



  

    let content = input_message.content.clone().unwrap();
    
    let masq: Masquerade  = Masquerade {
        avatar: Some("https://autumn.revolt.chat/avatars/tpDMc0zLiHzg9ZBLCHwF-7a50PRWwy9dsUMZaGi_2m".to_string()),
        name: Some("Reywen-MASQ".to_string()),
        colour: None,            
    };
    
     let message = match &content as &str {
       "test hello" => ReywenSys::Send{ content: ("World")},
       "test masq" => ReywenSys::SendMasq { content: ("hewo"), masq: (masq)},
       __ => ReywenSys::None,
    };    
    
    
    message.run(auth, input_message.clone()).await;
  
}

