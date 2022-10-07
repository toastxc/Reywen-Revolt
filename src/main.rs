// quark structs
mod lib {
    pub mod message;
    pub mod user;
    pub mod conf;
}

use crate::lib::{
    message::RMessage,
    conf::{MainConf, Auth}
};
// reywen fs
mod fs;
use fs::{conf_init};

// RevX2
pub mod rev_x;
use rev_x::*;

// reywen lib
mod lreywen;
use lreywen::*;

// network
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;


use std::str::from_utf8;

use tokio;

const PING: &str = r#"{
    "type": "Ping",
    "data": 0
}"#;


#[tokio::main]
async fn main()  {

    println!("booting...");

    // import
    let details_in = conf_init();

    let details = match details_in {
        Err(_main_conf) => panic!("failed to import json"),
        Ok(main_conf) => main_conf,
    };

    if details.message.message_enabled == false && details.bridge.bridge_enabled == false {
        panic!("No services enabled, reywen shutting down")
    }else {
        if details.message.message_enabled == true {
            println!("init: message")
        }if details.bridge.bridge_enabled == true {
            println!("init: bridge")
        };
    };


    let token = details.auth.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");

    websocket(url, details).await;

}

// establishes websocket connection
pub async fn websocket(url: String, details: MainConf) {


     let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
     println!("init: websocket");


     // this cant be in a program, because moving it requires defining an invalid type
     // and it cant be here because then the process is async do idk what to do pwp
     //tokio::time::sleep(Duration::from_secs(30));
     //ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(PING.to_string()));

     let (mut _write, read) = ws_stream.split();

     let read_future = read.for_each(|message| async {
        
        
        let data = message.unwrap().into_data();
        let out = from_utf8(&data).unwrap().to_string();

       // moved websocket main to self contained function for ease of use 

        newmain(out, details.clone()).await;
     });

    read_future.await;
}


// websocket main
// imports messages, cleans them and sends to 
// bridge and message processing
pub async fn newmain(out: String, details: MainConf) {

    let raw_message = rev_message_in(out);

    let (message, message2) = match raw_message {
        Err(_) => return,
        Ok(_) => (raw_message.as_ref().expect("REASON").clone(), raw_message.unwrap())
    };


    tokio::join!(
        br_main(details.clone(), message2),
        message_process(details.clone(), message),
        );
}



// main message engine 
pub async fn message_process(details: MainConf, message_in: RMessage) {

    if details.message.message_enabled == false {
        return
    };

    let content = message_in.content.clone();
    // validity test
    if content == None {
        return
    }else if message_in.author == details.auth.bot_id {
        return
    };
    let message = rev_message_clean(message_in).await;

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };

  
    match &content_vec[0] as &str {
        
        "?Mog" | "?mog"  => send(details.auth, message, ":01G7MT5B978E360NB6VWAS9SJ6:".to_string()).await,
        "?ver" | "?version" => send(details.auth, message, "**Version**\nReywen: `2`\nRevX: `2`".to_string()).await,
        "?echo" => send(details.auth, message, content_min1).await,
        "?sendas" => sendas(details.auth, message, content_vec).await,
        _ => return
    };

}

