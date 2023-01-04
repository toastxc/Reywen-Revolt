// quark structs
mod lib {
    pub mod message;
    pub mod user;
    pub mod conf;
}
use crate::lib::{
    message::RMessage,
    conf::Auth};


// reywen plugins
mod plugins {
    pub mod lreywen;
    pub mod message;
    pub mod shell;
    pub mod bridge;
    pub mod plural;
    pub mod e6;
    pub mod oop;


}
use crate::plugins::{
    //lreywen::*,
    lreywen::{send, reyshell_masq},
    message::message_process,
    shell::shell_main,
    bridge::br_main,
    plural::plural_main,
    e6::e6_main,
    oop::oop_main,
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
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;

use std::str::from_utf8;

use tokio::time::Duration;


const PING: &str = r#"{
    "type": "Ping",
    "data": 0
}"#;


#[tokio::main]
async fn main()  {

    println!("booting...");
    
    let details:Auth = conf_init()
        .expect("Failed to import config/reywen.json");

    let url:String = format!("wss://ws.revolt.chat/?format=json&version=1&token={}", details.token);    
        
    websocket(url, details).await; 

}

// establishes websocket connection
pub async fn websocket(url: String, details: Auth) {


     let (ws_stream, _response) = connect_async(url.clone()).await.expect("Failed to connect (websocket)");
     let (ws_stream_ping, _response) = connect_async(url.clone()).await.expect("Failed to connect (websocket)");

     println!("init: websocket");

     
     tokio::join! ( 
         websocket_sub(ws_stream, details),
         webocket_ping(ws_stream_ping),
         );
}

pub async fn webocket_ping(mut ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>) {

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let send_res = ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(PING.to_string())).await;
        
        if send_res.is_err() {
            panic!("Failed to ping websocket, restarting");
        };
    };

}

pub async fn websocket_sub(ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, details: Auth) {
    
          
    let (mut _write, read) = ws_stream.split();

     
    let read_future = read.for_each(|message| async {

        let data = match message {
             Ok(p) => {p.into_data()},
             Err(e) => {panic!("{e}")},
         };
        
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

    let (message, message2, message3, message4, message5, message6) = match raw_message {
        Err(_) => return,
        Ok(_) => (
            raw_message.as_ref().expect("failed converting message").clone(), 
            raw_message.as_ref().expect("failed converting message").clone(), 
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
        plural_main(details.clone(), message4),
        e6_main(details.clone(), message5),
        oop_main(details.clone(), message6)
        
        
        );
}

