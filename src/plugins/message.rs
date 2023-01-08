// external
use serde::{Serialize, Deserialize};

// internal
use crate::{fs_str, structs::{auth::Auth, message::RMessage}, lib::{lreywen::send, mongo::{RMongo, mongo_db}}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageConf {
    pub enabled: bool
}

// main message engine 
pub async fn message_process(details: Auth, message_in: &RMessage) {

    let conf = fs_str("config/message.json").expect("failed to read config/message.json\n{e}");

    let message_conf: MessageConf = serde_json::from_str(&conf)
            .expect("Failed to deser message.json");


    if !message_conf.enabled  { return };

    let content = &message_in.content;
    
    if content.is_none() {
        return
    }else if message_in.author == details.bot_id {
        return
    };
  
    let content_vec: Vec<&str> =  (message_in.content).as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    match content_vec[0] as &str {

        "?Mog" | "?mog"  => send(&details.token, message_in, ":01G7MT5B978E360NB6VWAS9SJ6:").await,
        "?ver" | "?version" => send(&details.token, message_in, "**Version**\nReywen: `2`\nRevX: `2`").await,
        _ => ()
    };


    // beyond here is mongo only

    if content_vec[0] != "mongotest" { return };

   

    
}

