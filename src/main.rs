// plugins
mod plugins {
    pub mod bridge;
    pub mod e6;
    pub mod message;
    pub mod plural;
    pub mod shell;

}
use crate::plugins::{
    bridge::br_main,
    e6::e6_main,
    message::message_process,
    plural::plural_main,
    shell::shell_main,
};


// libraries
mod lib {
    pub mod fs;
    pub mod lreywen;
    pub mod oop;
    pub mod rev_x;
    pub mod mongo;
}
use crate::lib::fs::*;
use crate::lib::rev_x::rev_message_in;


// structs
mod structs {
    pub mod auth;
    pub mod message;
    pub mod user;
}
use structs::auth::Auth;


// external crates
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, WebSocketStream};
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
        
        
        send_res.expect("Failed to ping websocket, restarting");
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
    
    if raw_message.is_err() {return};
    
    let message = raw_message.expect("failed to process ws message");


    tokio::join!(
        br_main(details.clone(), &message),
        message_process(details.clone(), &message),
        shell_main(details.clone(), &message),
        plural_main(details.clone(), &message),
        e6_main(details.clone(), &message),
    );
        
    
        
        
    
}

