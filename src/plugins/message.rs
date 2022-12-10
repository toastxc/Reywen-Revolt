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


    if message.enabled == false {
        return
    };

    let content = message_in.content.clone();
    // validity test
    if content == None {
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

    match &content_vec[0] as &str {

        "?Mog" | "?mog"  => send(details, message, ":01G7MT5B978E360NB6VWAS9SJ6:".to_string()).await,
        "?ver" | "?version" => send(details, message, "**Version**\nReywen: `2`\nRevX: `2`".to_string()).await,
        "?echo" =>  send(details, message, content_min1).await,
        "?sendas" => sendas(details, message, content_vec).await,
        _ => return
    };

}

