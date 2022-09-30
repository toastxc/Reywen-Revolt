// serde
//use serde::{Deserialize, Serialize};
use serde_json::{Result};


// structs
#[path = "./lib/message.rs"]
mod message;
use message::*;

#[path = "./lib/auth.rs"]
mod auth;
use auth::*;



// non functional - issue #18
// RevX2
//mod rev_x;
//use rev_x::*;


// misc
use rand::Rng;

// network
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// std
use std::{
    io::Read, 
    fs::File, 
//    thread, 
//    time, 
    str::from_utf8
};

use tokio;


// debug serde & file system read for config.json
fn conf_file() -> Result<Auth> {

    let mut config_json = File::open("config.json")
        .expect("File not found");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}

// debug serde message processor
fn message_in(raw: String) -> Result<RMessage> {


    let message: Result<RMessage> = serde_json::from_str(&raw);

    match message {
        Err(rmessage) => Err(rmessage),
        Ok(ref _rmessage) =>  Ok(message.unwrap())
    }
    
}

#[tokio::main]
async fn main()  {


    let data_in = conf_file();
    let data = match data_in {

        Ok(auth) => auth,
        Err(error) => panic!("Invalid credentials, {error}")
    };

    if data.token == "" {
        panic!("Invalid credentials, bot requires a token");
    }else if data.bot_id == "" {
        panic!("Invalid credentials, bot requires an ID");
    }else if data.sudoers[0] == "" {
        println!("no sudoers found")
    };

    let token = data.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");

     websocket(url, data).await;

}

// establishes web socket conneciton
pub async fn websocket(url: String, authen: Auth) {


     let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
     println!("WebSocket handshake has been successfully completed");


     let (mut write, read) = ws_stream.split();

    write.send(Message::Text(r#"{
        "type": "ping",
        "data": 0
      }"#.to_string()+"\\n")).await.unwrap();

    let read_future = read.for_each(|message| async {
         
        let data = message.unwrap().into_data();
       
        let out = from_utf8(&data).unwrap().to_string();


        // new main!
        
       newmain(authen.clone(), out).await;

    });

    read_future.await;


}

// moved from websocket to avoid confusion
// debugs and sends messages to the engine
pub async fn newmain(authen: Auth, out: String) {

    let inval_message = message_in(out);
    
    match inval_message {
        Err(_) => return,
        Ok(_) => print!("")

    };

    let message = message_clean(inval_message.unwrap());
  
    message_process(authen, message).await;

}

// main message engine 
pub async fn message_process(data: Auth, message_in: RMessage) {

    
    let content = message_in.content.clone();

    if content == None {
        return
    };

    let message = message_clean(message_in);

    let content = message.content.clone().unwrap();

    let reply = message.author.clone();

    if content == "?hello Reywen".to_string() {
  
        rev_send(data, message, format!("hello <@{}>", reply).to_string()).await;

    };

}

// sends messages over http


pub async fn rev_send(auth: Auth, message: RMessage, content: String)  {
    
    let reply = RReplies {
        id: message._id,
        mention: false,
    };

    let payload = RMessagePayload {

        content: Some(content),
      //  replies: None,
        
        replies: Some(vec![reply]),
          attachments: None,
          masquerade: None
    };

    let payload2 = serde_json::to_string(&payload).unwrap();

    println!("rev_send...");

    let channel = message.channel;

    let mut random = rand::thread_rng();
    let idem: i64 = random.gen();

    let client: std::result::Result<reqwest::Response, reqwest::Error> = 
        reqwest::Client::new()
        .post(format!("https://api.revolt.chat/channels/{channel}/messages"))
        .header("x-bot-token", auth.token)
        .header("Idempotency-Key", idem)
        .header("Content-Type", "application/json")
        .body(payload2)
        .send().await;
 
    match client {
        Ok(_) => println!("{}", client.unwrap().text().await.unwrap()),
        Err(_) => println!("{:?}", client)
    };
}


// cleans invalid characters such as \n and \
pub fn message_clean(mut message: RMessage) -> RMessage {

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
