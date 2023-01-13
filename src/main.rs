// plugins
mod plugins {
    pub mod bridge;
    pub mod e6;
    pub mod message;
    pub mod plural;
    pub mod shell;
}
use crate::plugins::{
    bridge::br_main, e6::e6_main, message::message_main, plural::plural_main, shell::shell_main,
};

// libraries
mod lib {
    pub mod fs;
    pub mod lreywen;
    pub mod mongo;
    pub mod oop;
    pub mod rev_x;
    pub mod websocket;
}
use crate::lib::fs::*;
use crate::lib::rev_x::rev_message_in;
use crate::lib::websocket::from_ws;
use lib::websocket::RWebsocket;

// structs
mod structs {
    pub mod auth;
    pub mod message;
    pub mod user;
}

// external crates
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    println!("booting...");

    let auth = conf_init().expect("Failed to import config/reywen.json");
    let ws_conf = ws_init().expect("failed to import ws from config/reywen.json");

    let websocket = RWebsocket::new(ws_conf);


    loop {
        println!("starting websocket");
        websocket
            .clone()
            .generate()
            .await
            .for_each(|message| async {
                let data = from_ws(message);
                if data.is_empty() {
                    println!("Caught ws error exiting websocket");
                    return;
                };
                let input = rev_message_in(data);

                let input_message = match input {
                    Err(_) => return,
                    Ok(a) => a,
                };

                tokio::join!(
                    br_main(auth.clone(), &input_message),
                    e6_main(auth.clone(), &input_message),
                    message_main(auth.clone(), &input_message),
                    plural_main(auth.clone(), &input_message),
                    shell_main(auth.clone(), &input_message),
                );
            })
            .await;
    }
}