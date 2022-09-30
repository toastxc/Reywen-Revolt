// filesystem
use std::io::Read;
use std::fs::*;

// serde
use serde::{Deserialize, Serialize};
use serde_json::{Result};


// web socket 
//use url::Url;
//use tungstenite::{connect, Message};

// internal
#[path = "./structure/message.rs"]
mod message;
use message::*;

use tokio;
use tokio::io::AsyncWriteExt;


use std::{thread, time};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use rand::Rng;

use futures_util::{StreamExt, SinkExt};

use std::str::from_utf8;

use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {

    token: String,
    bot_id: String,
    sudoers: Vec<String>,

}

fn conf_file() -> Result<Auth> {

    let mut config_json = File::open("config.json")
        .expect("File not found");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}

fn Message_in(raw: String) -> Result<RMessage> {


    let message: Result<RMessage> = serde_json::from_str(&raw);

    match message {
        Err(RMessage) => Err(RMessage),
        Ok(ref RMessage) =>  Ok(message.unwrap())
    }

    
}

#[tokio::main]
async fn main()  {


     let data_in = conf_file();
    let data = match data_in {

        Ok(Auth) => Auth,
        Err(error) => panic!("Invalid credentials, {error}")
    };

    let token = data.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");



     websocket(url, data).await;

}


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
        //let p_data = tokio::io::stdout().write(&data).await.unwrap();
       
        let out = from_utf8(&data).unwrap().to_string();
        //println!("{:?}", out);


        // new main!
        
       newmain(authen.clone(), out).await;

        //message_process(authen.clone(), Message_deser.expect("failed to unwrap"));
    });

    read_future.await;


}

/*     
        let Message_deser = Message_in(write);

        match Message_deser {
            Err(RMessage) => return Err(RMessage),
            Ok(ref RMessage) => message_process(data.clone(), Message_deser.unwrap())
        };

        Ok(())

}
*/


pub async fn newmain(authen: Auth, out: String) {

    let inval_message = Message_in(out);

    
    match inval_message {
        Err(_) => return,
        Ok(_) => print!("")

    };

    let message = message_clean(inval_message.unwrap());
  
    message_process(authen, message).await;

}


pub async fn message_process(data: Auth, message_in: RMessage) {

    
    let content = message_in.content.clone();

    if content == None {
        return
    };

    let message = message_clean(message_in);

    let content = message.content.clone().unwrap();

    let reply = message.author.clone();

   // println!("{}", message.content.as_ref().expect("failed to unwrap debug"));

    if content == "?hello Reywen".to_string() {
  
        rev_send(data, message, format!("hello <@{}>", reply).to_string()).await;

    };

    /*
      RMessage { 
      _id: "01GE1FJJW76ZE487Y6F5PQ8Y7V", 
      nonce: Some("01GE1FJJANPKTPB0C5R7HCGSVW"), 
      channel: "01G1JBX8J5GRVMWH4NY3QWBQ3B", 
      author: "01FSRTTGJC1XJ6ZEQJMSX8Q96C", 
      content: Some("h"), 
      mentions: None, 
      replies: None, 
      masquerade: None }

      */

}

pub async fn rev_send(auth: Auth, message: RMessage, content: String)  {

    
    let reply = RReplies {
        id: message._id,
        mention: false,
    };

    let payload = RMessage_payload {

        content: Some(content),
      //  replies: None,
        
        replies: Some(vec![reply]),
          attachments: None,
          masquerade: None
    };

    let payload2 = serde_json::to_string(&payload).unwrap();

    println!("{}", payload2);
    
    println!("sending...\n\n");

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
