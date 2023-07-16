use futures_util::{SinkExt, StreamExt};
use reywen::{
    client::{methods::user::DataEditUser, Client},
    structures::{authentication::login::DataLogin, users::UserStatus},
    websocket::data::{WebSocketEvent, WebSocketSend},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSendAuth {
    email: String,
    password: String,
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    // import auth from file
    let auth = serde_json::from_str::<DataSendAuth>(include_str!("config/vlod_auth.json"))
        .expect("Invalid JSON for Auth config");

    // if there is no token, create one and sign in - otherwise use existing token
    let client = match auth.token {
        Some(token) => Client::from_token(&token, false).unwrap(),
        None => { Client::from_login(&DataLogin::non_mfa(&auth.email, &auth.password)) }
            .await
            .unwrap(),
    };

    // for every websocket event
    loop {
        let (mut read, write) = client.websocket.dual_async().await;

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
