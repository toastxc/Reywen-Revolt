// structs

use crate::{
    lib::{
        message::{
            RMessage, RMessagePayload, RReplies, 
           // Masquerade
        },
            user::RUserFetch},
//        MainConf,
        Auth};


// dep
use rand::Rng;
use serde_json::{Result};


pub async fn sudocheck(user: String, auth: Auth) -> bool {

  
    for x in 0..auth.sudoers.len() {
        if user == auth.sudoers[x] {
            return true
        };
    };

    return false

}

// deserializes websocket messages
pub fn rev_message_in(raw: String) -> Result<RMessage> {


    let message: Result<RMessage> = serde_json::from_str(&raw);

    match message {
        Err(rmessage) => Err(rmessage),
        Ok(ref _rmessage) =>  Ok(message.unwrap())
    }
}


// cleans invalid characters such as \n and \
pub async fn rev_message_clean(mut message: RMessage) -> RMessage {

    if message.content == None {
        return message
    };

    let mut out = String::new();

    let iter = message.content.as_ref().unwrap().chars().count();
    let content = message.content.as_ref().unwrap();


    for x in 0..iter {
        let current = content.chars().nth(x);

        if current == Some('\n') {
            out += "\\n";
        }else if current == Some('\\') {
            out += "\\\\";
        }else {
            out += &current.unwrap().to_string();  
        };
    };
    message.content = Some(out);
    return message    
}

pub async fn rev_user(auth: Auth, target: String)   -> Result<RUserFetch> {

  //  println!("rev: user");
   
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new() 
    .get(format!("https://api.revolt.chat/users/{target}"))
    .header("x-bot-token", auth.token)
    .send().await;

    let client_res = match client {
        Ok(_) => client.unwrap().text().await.unwrap(),
        Err(_) => "Err:\n{error}".to_string() 
    };   
        
  
  //  println!("{:?}", client_res);

    let message: Result<RUserFetch> = serde_json::from_str(&client_res);
    match message {
        Ok(_) => return Ok(message.unwrap()),
        Err(_) => return message
    };

}       

pub async fn rev_send(auth: Auth, message: RMessage, payload: RMessagePayload)  {

    let channel = message.channel;

    let mut random = rand::thread_rng();
    let idem: i64 = random.gen();

    let payload2 = serde_json::to_string(&payload).unwrap();

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .post(format!("https://api.revolt.chat/channels/{channel}/messages"))
        .header("x-bot-token", auth.token)
        .header("Idempotency-Key", idem)
        .header("Content-Type", "application/json")
        .body(payload2)
        .send().await;

    match client {
        Ok(_) => return,
        Err(_) => println!("Err:\n{:?}", client)
    };
}


pub async fn rev_del(auth: Auth, message: RMessage) {
    
    let channel = message.channel;
    let target = message._id;
        
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .delete(format!("https://api.revolt.chat/channels/{channel}/messages/{target}"))
    .header("x-bot-token", auth.token)
    .send().await;
    
     match client {
        Ok(_) => return,
        Err(_) => println!("Err:\n{:?}", client)
    };    
            
}




// converts websocket replies to API compatible replies
pub async fn rev_convert_reply(input: Option<Vec<String>>) -> Option<Vec<RReplies>> {

    if input == None {

        return None

    }else {

        let mut repstruct = vec![];
        let iter = input.clone()?.len();

        for x in 0..iter {

            let input_iter = &input.as_ref().expect("failed to convert input wstoapi")[x];

            let reply = RReplies {
                id: input_iter.to_string(),
                mention: false,
            };
            repstruct.push(reply);
        };

        return Some(repstruct)
    };

}
