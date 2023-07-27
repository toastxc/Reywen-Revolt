use futures_util::{SinkExt, StreamExt};
use reywen::{
    client::{methods::user::DataEditUser, Client},
    structures::{
        authentication::{login::ResponseLogin, mfa::MFAResponse, session::Session},
        users::UserStatus,
    },
    websocket::data::{WebSocketEvent, WebSocketSend},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSendAuth {
    email: String,
    password: String,
    token: Option<String>,
    mfa_totp: Option<String>,
}

#[tokio::main]
async fn main() {
    // import auth from file
    println!("import conf file");
    let mut auth = serde_json::from_str::<DataSendAuth>(include_str!("config/vlod_auth.json"))
        .expect("Invalid JSON for Auth config");

    // if there is no token, create one and sign in - otherwise use existing token
    if auth.token.is_none() {
        println!("no token found! generating...");
        if let ResponseLogin::Success(Session { token, .. }) = Client::session_login_smart(
            &auth.email,
            &auth.password,
            Some(MFAResponse::totp(&auth.clone().mfa_totp.unwrap())),
        )
        .await
        .unwrap()
        {
            auth.token = Some(token);
            std::fs::write(
                "examples/config/vlod_auth.json",
                serde_json::to_string_pretty(&auth).unwrap(),
            )
            .unwrap();
            println!("done generating, restart");
        }
        return;
    }

    println!("making client");
    let client = Client::from_token(&auth.token.unwrap(), false).unwrap();

    // for every websocket event
    loop {
        let (mut read, write) = client.websocket.dual_async().await;
        println!("connected to websocket");
        while let Some(input) = read.next().await {
            let write = Arc::clone(&write);
            let client = client.clone();

            tokio::spawn(async move {
                match input {
                    WebSocketEvent::Error { .. } => {}
                    WebSocketEvent::Ready { servers, .. } => {
                        let user = client
                            .user_fetch_self()
                            .await
                            .expect("Could not find information for current user.");

                        client
                            .user_edit(
                                &user.id,
                                &DataEditUser::new().set_status(
                                    UserStatus::new()
                                        .set_text(&format!("servers: {}", servers.len())),
                                ),
                            )
                            .await
                            .ok();

                        write
                            .lock()
                            .await
                            .send(WebSocketSend::ping(0).into())
                            .await
                            .ok();
                    }
                    WebSocketEvent::Message { message } => {
                        println!("{message:?}") // or add it to your UI if you're creating a custom Revolt client.
                    }
                    WebSocketEvent::Pong { .. } => {
                        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                        write
                            .lock()
                            .await
                            .send(WebSocketSend::ping(0).into())
                            .await
                            .ok();
                    }
                    _ => {}
                }
            });
        }
    }
}
