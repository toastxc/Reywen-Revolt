use crate::{Auth, RMessage, send, sendas};

use serde::{Serialize, Deserialize};

use crate::fs_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageConf {

    pub enabled: bool
}

// main message engine 
pub async fn message_process(details: Auth, message_in: RMessage) {

    let conf = fs_str("config/message.json").expect("failed to read config/message.json\n{e}");

    let message: MessageConf = serde_json::from_str(&conf)
            .expect("Failed to deser message.json");


    if !message.enabled  { return };

    let content = message_in.content.clone();
    
    if content.is_none() {
        return
    }else if message_in.author == details.bot_id {
        return
    };
    let message = message_in;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };

    match content_vec[0] as &str {

        "?Mog" | "?mog"  => send(&details.token, &message, ":01G7MT5B978E360NB6VWAS9SJ6:").await,
        "?ver" | "?version" => send(&details.token, &message, "**Version**\nReywen: `2`\nRevX: `2`").await,
        "?echo" =>  send(&details.token, &message, &content_min1).await,
        "?sendas" => sendas(&details.token, &message, &content_vec).await,
        _ => ()
    }
}

