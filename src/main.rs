// quark structs
mod lib {
    pub mod auth;
    pub mod message;
    pub mod user;
    pub mod br;
}

use crate::lib::{
    message::{RMessage, RMessagePayload, Masquerade},
    br::BrConf,
    auth::Auth,
};

// reywen fs
mod fs;
use fs::{auth_init, bridge_init};

// RevX2
pub mod rev_x;
use rev_x::*;

// reywen lib
mod lreywen;
use lreywen::*;

// network
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};


use std::str::from_utf8;
use tokio;

#[tokio::main]
async fn main()  {

    println!("booting...");
    // auth files
    let data_in = auth_init();
    let data = match data_in {
        Ok(auth) => auth,
        Err(error) => panic!("Invalid credentials, {error}")
    };

    if data.token == "" {
        panic!("Invalid credentials, bot requires a token");
    }else if data.bot_id == "" {
        panic!("Invalid credentials, bot requires an ID");
    }else if data.sudoers[0] == "" {
        println!("WARN: no sudoers found")
    };
    println!("init: auth.json");
    
    

    // bridge
    let br_in = bridge_init();
    let br = match br_in {
        Err(_br) => panic!("failed to import bridge conf\n"),
        Ok(br) => br
    };
    if br.enabled == true {
        println!("init: bridge.json");
        if br.channel_1.len() != 26{
            println!("WARN: bridge channels may not be valid");
        }else if br.channel_2.len() != 26 {
            println!("WARN: bridge channels may not be valid");
        }else if br.channel_1 == br.channel_2 {
            panic!("bridge channels cannot be the same")
        };
    };

    let token = data.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");

 
     websocket(url, data, br).await;

}

// establishes websocket connection

pub async fn websocket(url: String, authen: Auth, br: BrConf) {

     let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
     println!("init: websocket");

     let (mut write, read) = ws_stream.split();

    write.send(Message::Text(r#"{
        "type": "ping",
        "data": 0
      }"#.to_string()+"\\n")).await.unwrap();

    let read_future = read.for_each(|message| async {
         
        let data = message.unwrap().into_data();
       
        let out = from_utf8(&data).unwrap().to_string();

       // moved websocket main to self contained function for ease of use 
       newmain(authen.clone(), out, br.clone()).await;
    });

    read_future.await;
}



// websocket main
// imports messages, cleans them and sends to 
// bridge and message processing
pub async fn newmain(authen: Auth, out: String, br: BrConf) {

    let inval_message = rev_message_in(out.clone());
    let inval2 = rev_message_in(out.clone());

    match inval_message {
        Err(_) => return,
        Ok(_) => print!("")

    };

    
    let message = inval_message;
    
    message_process(authen.clone(), message.expect("failed to process message")).await;
    
    // br does not require 'cleaned' messages
    if br.enabled == true {
        br_main(authen.clone(), inval2.unwrap(), br).await;
    };

}
// main for bridge, WIP
pub async fn br_main(auth: Auth, input_message: RMessage, br: BrConf) {


    let (chan1, chan2) = (br.channel_1, br.channel_2);
    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade != None {
        return
    };


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
        
        let user = rev_user(auth.clone(), input_message.author.clone()).await.expect("failed to GET user details");

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

// main message engine 
pub async fn message_process(data: Auth, message_in: RMessage) {
 
    let content = message_in.content.clone();

    // validity test
    if content == None {
        return
    }else if message_in.author == data.bot_id {
        return
    };

    let message = rev_message_clean(message_in).await;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };
 
    match &content_vec[0] as &str {
        
        "?Mog" | "?mog"  => send(data, message, ":01G7MT5B978E360NB6VWAS9SJ6:".to_string()).await,
        "?ver" | "?version" => send(data, message, "**Version**\nReywen: `2.0.1`\nRevX: `2.0.2`".to_string()).await,
        "?echo" => send(data, message, content_min1).await,
        "?sendas" => sendas(data, message, content_vec).await,
        _ => return
    };

}

// masq wrapper for rev_send

pub async fn sendas(auth: Auth, message: RMessage, content_vec: Vec<&str>) {

    if content_vec.len() < 3 {
        send(auth, message, "invalid use of sendas".to_string()).await;
        return
    };
    //let from = message._id.clone();
    let masq = content_vec[1];
    let mut content = String::new();
    //content = "placeholder".to_string();

    let link = match masq {
        "bingus"    | "cheese"  | "dad" | 
        "deeznuts"  |  "insert" | "joe_biden" | 
        "valence"   | "walter"  | "woof" => format!("https://toastxc.xyz/TXCS/{masq}.jpg"),
        _ => format!("https://toastxc.xyz/TXCS/default.png")
    };

    for x in 0..content_vec.len() -2 {
        content += &format!(" {}", content_vec[x + 2]);
    };

    let masq_s = Masquerade {
        name: Some(masq.to_string()),
        avatar: Some(link),
        colour: None,
    };

 
    let replier = rev_convert_reply(message.replies.clone()).await;

    let returner = RMessagePayload {
          content: Some(content),
          replies: replier,
          attachments: None,
          masquerade: Some(masq_s)
    };

    rev_send(auth.clone(), message.clone(), returner).await;
    rev_del(auth.clone(), message.clone()).await;
}
