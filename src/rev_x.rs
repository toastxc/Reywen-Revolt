use crate::{
        lib::{
        message::{
            RMessage, RMessagePayload, RReplies, },
            user::RUserFetch},
        Auth};

use rand::Rng;

// given a user ID, checks if the user is a 'sudoer' or not 
pub async fn sudocheck(user: String, auth: Auth) -> bool {

    for x in 0..auth.sudoers.len() {
        if user == auth.sudoers[x] {

            println!("WARN: SUDOER ACTION");
            return true;
        };
    };
    false
}

// deserializes websocket messages
pub fn rev_message_in(raw: String) -> Result<RMessage, serde_json::Error> {

    let message: Result<RMessage, serde_json::Error> = serde_json::from_str(&raw);

    match message {
        Err(e) => Err(e),
        Ok(a) =>  Ok(a)
    }
}


// cleans invalid characters such as \n and \
// 
// This function was vital for RevX1 but is not needed for reqwest
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
    message    
}


// https://developers.revolt.chat/api/#tag/User-Information/operation/fetch_user_req
pub async fn rev_user(auth: Auth, target: String) -> Result<RUserFetch,  serde_json::Error> {

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new() 
        .get(format!("https://api.revolt.chat/users/{target}"))
        .header("x-bot-token", auth.token)
        .send().await;

    

    let client_res = match client {
        Ok(a) => a.text().await.unwrap(),
        Err(e) => e.to_string(),
    };   
     

    let message: Result<RUserFetch, serde_json::Error> = serde_json::from_str(&client_res);
    return match message {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    };


    // issue  27 

}       

// https://developers.revolt.chat/api/#tag/Messaging/operation/message_send_message_send
pub async fn rev_send(auth: Auth, message: RMessage, payload: RMessagePayload)  {

    let mut random = rand::thread_rng();
    let idem: i64 = random.gen();

    let payload2 = serde_json::to_string(&payload).unwrap();

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .post(format!("https://api.revolt.chat/channels/{}/messages", message.channel))
        .header("x-bot-token", auth.token)
        .header("Idempotency-Key", idem)
        .header("Content-Type", "application/json")
        .body(payload2)
        .send().await;

    http_err(client, "REV_SEND");
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
    if http.as_ref().unwrap().status().is_success() == false {
    println!("{message}_HTTP_ERROR: {}", http.unwrap().status());
    };
}

// https://developers.revolt.chat/api/#tag/Messaging/operation/message_delete_req
pub async fn rev_del(auth: Auth, message: RMessage) {
    
    let channel = message.channel;
    let target = message._id;
        
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .delete(format!("https://api.revolt.chat/channels/{channel}/messages/{target}"))
    .header("x-bot-token", auth.token)
    .send().await;
     
    http_err(client, "REV_DEL");

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
