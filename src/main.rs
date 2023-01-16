// external crates

use futures_util::StreamExt;
use reywen::{
    bonfire::bonfire::from_ws,
    delta::{
        delta::rev_message_in,
        fs::{conf_init, ws_init},
    },
    plugins::{
        bridge::br_main, e6::e6_main, message::message_main, plural::plural_main, shell::shell_main,
    },
    quark::bonfire::RWebsocket,
};

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
                    println!("reseting websocket");
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