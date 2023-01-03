
// serde is the toolchain for interpreting data from jsons
use serde::{Deserialize, Serialize};
// these are all internal libraries
use crate::{fs::fs_str, lib::{conf::Auth, message::{RMessage, RMessagePayload}}};
use super::oop::Reywen;

// the configuration for a basic plugin, optional_channel does not have to exist but enabled does.
#[derive(Debug, Deserialize, Serialize)]
struct MogConf {
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_channel: Option<String>,
    enabled: bool,
}

// main function for plugin
// this will activate when a message is received from websocket
pub async fn e6_main(auth: Auth, input_message: RMessage) {
    
     // imports the json from config/mogus.json
     let conf: String = fs_str("config/mogus.json").expect("failed to read config mogus.json\n{e}");
     let mogus_conf: MogConf = serde_json::from_str(&conf).expect("Failed to deser mogus.json{e}");
     
     // if mogus is disabled, close function
     if !mogus_conf.enabled {
         return
     
     // if websocket message is empty, close function    
     }else if input_message.content.is_none() {
         return
     };
     // creates a vector of messages from input message
     let temp = input_message.content.unwrap();
     let convec: Vec<&str> = temp.split(' ').collect();
     
    
    // now, to create a new client
    let client = Reywen::new(auth);
    
    // the Revolt api offers many different options for message payloads, this is a simple one
    let payload = RMessagePayload::new()
        .content("I love amogus");
    
    // if the first message someone sends is 'amogus', send ^ payload
    if convec[0] == "?amogus" {
        client.clone().send(payload, &input_message.channel).await;
    };
   
    
    // same example with a match
    let word: &'static str = match convec[0] {
        "?amogi" => "amongus",
        "?mogus" => "mogmogmog",
        _ => "", 
        
    }; 
    
    let newpayload = RMessagePayload::new().content(&word);
       
    client.send(newpayload, &input_message.channel).await;
    
    // this is a very new and experimental way of using reywen... and as you can see its not perfect yet
    // more methods will be derived and ported over, as well as more abstration

}
