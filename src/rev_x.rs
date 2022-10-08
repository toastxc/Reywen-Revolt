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
        //Ok(_) => println!("{}", client.unwrap().text().await.unwrap()),
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
/*
//pub fn conf_error(details_in: 
pub async fn br_main(details: MainConf, input_message: RMessage) {

    let auth = details.auth.clone();
    let br = details.bridge.clone();

    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade != None {
        return
    };


    let (chan1, chan2) = (br.channel_1, br.channel_2);


    // channel switch
    let mut chan_rec = String::new();
    if input_message.channel == chan1 {
       chan_rec = chan2;
    }else if input_message.channel == chan2 {
       chan_rec = chan1;
    };


    
    let mut message = input_message.clone();
    
    message.channel = chan_rec;

    let mut br_masq = Masquerade {
        name: None,
        avatar: None,
        colour: None
    };
   
    // masq switch - if user has no masquerade: pull from user info API
    // else - port over masquerade details 
    if input_message.masquerade == None {

        // API get masq
        
        let user1 = rev_user(auth.clone(), input_message.author.clone()).await;

        let user = match user1 {
            Ok(_) => user1.expect("failed to GET user details"),
            Err(_)  => return
        };

        let pfplink = user.avatar.unwrap().id;

        let pfp = format!("https://autumn.revolt.chat/avatars/{pfplink}");

        br_masq = Masquerade {
            name: Some(user.username),
            avatar: Some(pfp),
            colour: None
        };
        
    }else {
        
        // translate masq
        br_masq = Masquerade {
            name: message.masquerade.as_ref().unwrap().name.clone(),
            avatar: message.masquerade.as_ref().unwrap().avatar.clone(),
            colour: None
        };  

    };

    // message for rev_send
    let payload = RMessagePayload {
        content: message.content.clone(),
        attachments: None,
        replies: rev_convert_reply(input_message.replies).await,
        masquerade: Some(br_masq),
    };

    rev_send(auth, message, payload).await;

}
*/
