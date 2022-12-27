  // a sandbox for experimenting with OOP design patterns for abstraction and DX
    // lets create a simple hello world thingo

use crate::{lib::{conf::Auth, message::{RMessage, Masquerade, RMessagePayload}}, rev_x::{rev_send, rev_del_2}};
use crate::send;

#[derive(Debug)]
enum ReywenSys{

    Send{content: String},
    SendMasq{content: String, masq: Masquerade},
    Delete{message: String},
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
            },
            
            Self::Delete{message} => {
                
                rev_del_2(&auth.token, &message_input.channel, message).await;
            },
            
            Self::None => {},
            
        };
    }


}

      let convec: Vec<String> = input_message.content.clone().unwrap().split_whitespace().map(str::to_string).collect();
      
  
    
    let masq: Masquerade  = Masquerade {
        avatar: Some("https://autumn.revolt.chat/avatars/tpDMc0zLiHzg9ZBLCHwF-7a50PRWwy9dsUMZaGi_2m".to_string()),
        name: Some("Reywen-MASQ".to_string()),
        colour: None,            
    };
   
     if convec.len() < 3 {
         return
     };
         
         
      
    
    let message = match &convec[1] as &str{
        "hello" => ReywenSys::Send{ content: ("World").to_string()},
        "masq" => ReywenSys::SendMasq { content: ("hewo").to_string(), masq: (masq)},
        "del" => ReywenSys::Delete { message: convec[2].clone() },
        _ => ReywenSys::None
    };
    
    message.run(auth, input_message).await;

}

