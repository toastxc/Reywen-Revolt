  // a sandbox for experimenting with OOP design patterns for abstraction and DX
    // lets create a simple hello world thingo

use crate::lib::{conf::Auth, message::{RMessage, Masquerade}};
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
                
            }
            Self::Delete => {},
            
            Self::None => {},
            
        };
    }

}



  

    let content = input_message.content.clone().unwrap();
    

     let message = match &content as &str {
       "greer" => ReywenSys::Send{ content: ("greer")},
       //"" => ReywenSys::Send{ content: ("balls")},
       __ => ReywenSys::None,
    };    
    
    
    message.run(auth, input_message.clone()).await;
    println!("{:?}", message);
}

