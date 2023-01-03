
use crate::RMessage;

use crate::lib::{
    user::RUserFetch,
    message::RMessagePayload,
    message::RReplies
};

use rand::Rng;

// given a user ID, checks if the user is a 'sudoer' or not 
pub fn sudocheck(user: &str, comment: &str, sudoers: &[String]) -> bool {

    if sudoers.contains(&user.to_string()) {
        println!("WARN: SUDOER ACTION FROM {user} in {comment}");
        return true
    };
    false 
        
    
}

// deserializes websocket messages
pub fn rev_message_in(raw: String) -> Result<RMessage, serde_json::Error> {

     serde_json::from_str(&raw)
}

// https://developers.revolt.chat/api/#tag/User-Information/operation/fetch_user_req
pub async fn rev_user(token: &str, target: &str) -> Option<RUserFetch> {

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new() 
        .get(format!("https://api.revolt.chat/users/{target}"))
        .header("x-bot-token", token)
        .send().await;


    if client.is_ok() {
        
        let client_res = client.unwrap().text().await.unwrap();
         
        let json:RUserFetch = serde_json::from_str(&client_res).unwrap();
         
        return Some(json)
        
    };
        
      http_err(client, "REV_USER_ERR");
        
        None
}       

// https://developers.revolt.chat/api/#tag/Messaging/operation/message_send_message_send
pub async fn rev_send(token: &str, channel: &str, payload: RMessagePayload)  {

    let mut random = rand::thread_rng();
    let idem: i64 = random.gen();

    let payload2 = serde_json::to_string(&payload).unwrap();

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .post(format!("https://api.revolt.chat/channels/{}/messages", channel))
        .header("x-bot-token", token)
        .header("Idempotency-Key", idem)
        .header("Content-Type", "application/json")
        .body(payload2)
        .send().await;

    http_err(client, "REV_SEND");
}

// https://developers.revolt.chat/api/#tag/Server-Members/operation/member_remove_req
pub async fn rev_kick(token: &str, user: &str, server: &str) {

    
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .delete(format!("https://api.revolt.chat/servers/{}/members/{}", server, user))
        .header("x-bot-token", token) 
        .send().await;
       
    http_err(client, "REV_KICK");
}


// for administrators
// prints http based error codes to stdout with an optional message
pub fn http_err(http: Result<reqwest::Response, reqwest::Error>, message: &str) {

    // reqwest
    match http {
        Ok(_) => {},
        Err(e) => {println!("{message}_REQWEST_ERROR:\n{e}"); return},
    };

    // http
    if !http.as_ref().unwrap().status().is_success() {
        println!("{message}_HTTP_ERROR: {}", http.unwrap().status());
    };
}

// DEPRICATED
pub async fn rev_del(token: &str, message: &RMessage) {
    
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .delete(format!("https://api.revolt.chat/channels/{}/messages/{}", message.channel, message._id))
    .header("x-bot-token", token)
    .send().await;
     
    http_err(client, "REV_DEL");
}

// https://developers.revolt.chat/api/#tag/Messaging/operation/message_delete_req
pub async fn rev_del_2(token: &str, channel: &str, message: &str) {
    
let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .delete(format!("https://api.revolt.chat/channels/{}/messages/{}", channel, message))
    .header("x-bot-token", token)
    .send().await;
     
    http_err(client, "REV_DEL");
}

// converts websocket replies to API compatible replies
pub async fn rev_convert_reply(input: Option<Vec<String>>) -> Option<Vec<RReplies>> {

    if input.is_some() {

        let mut repstruct = Vec::new();

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
    None
}
