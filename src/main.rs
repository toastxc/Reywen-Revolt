// quark structs
mod lib {
    pub mod message;
    pub mod user;
    pub mod conf;
}
use crate::lib::{
    message::RMessage,
    conf::{Auth}
};


// reywen plugins
mod plugins {
    pub mod lreywen;
    pub mod message;
    pub mod shell;
    pub mod bridge;
    pub mod plural;
}
use crate::plugins::{
    lreywen::*,
    message::*,
    shell::*,
    bridge::*,
    plural::*,
};


// reywen fs
mod fs;
use fs::{conf_init, fs_str};

// RevX2
pub mod rev_x;
use rev_x::*;


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


    let token = details.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");

    websocket(url, details).await;

}

// establishes websocket connection
pub async fn websocket(url: String, details: Auth) {


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

        new_main(out, details.clone()).await;
     });

    read_future.await;
}


// websocket main
// imports messages, cleans them and sends to 
// bridge and message processing
pub async fn new_main(out: String, details: Auth) {

    let raw_message = rev_message_in(out);

    let (message, message2, message3, message4) = match raw_message {
        Err(_) => return,
        Ok(_) => (
            raw_message.as_ref().expect("failed converting message").clone(), 
            raw_message.as_ref().expect("failed converting message").clone(), 
            raw_message.as_ref().expect("failed converting message").clone(),
            raw_message.as_ref().expect("failed converting message").clone()

            )
    };


    tokio::join!(

        br_main(details.clone(), message),
        message_process(details.clone(), message2),
        shell_main(details.clone(), message3),
        plural_main(details.clone(), message4)
        
        );
}

