#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures_util::{SinkExt, StreamExt};

    use crate::{
        client::Client,
        structures::authentication::{login::ResponseLogin, mfa::MFAResponse, session::Session},
        websocket::{
            data::{WebSocketEvent, WebSocketSend},
            WebSocket,
        },
    };

    // test of reywenv3 websocket owo
    // #[tokio::test]
    // pub async fn ws_stream_test() {
    //     let input = WebSocket::from_token("");
    //     // generate a websocket connection AND convert types
    //     let mut ws = input.to_owned().start().await;
    //
    //     // for every message
    //     while let Some(item) = ws.next().await {
    //         // if the event is a message
    //         if let WebSocketEvent::Message { .. } = item {
    //             return;
    //         }
    //     }
    // }

    // #[tokio::test]
    // pub async fn ws_full_test() {
    //     let ws = WebSocket::from_token("").dual_connection().await;
    //
    //     let (mut write, mut read) = ws;
    //
    //     while let Some(item) = read.next().await {
    //         // if the event is a message
    //         match item {
    //             WebSocketEvent::Message { .. } => {
    //                 write.send(WebSocketSend::ping(0).into()).await.ok();
    //             }
    //             WebSocketEvent::Pong { .. } => {
    //                 return;
    //             }
    //
    //             _ => {}
    //         };
    //     }
    // }

    #[tokio::test]
    pub async fn ws_test_async() {
        let ws = WebSocket::from_token("");

        let (mut read, write) = ws.dual_async().await.unwrap();

        while let Some(item) = read.next().await {
            let write = Arc::clone(&write);

            // if the event is a message
            tokio::spawn(async move {
                match item {
                    WebSocketEvent::Message { .. } => {
                        write
                            .write()
                            .await
                            .send(WebSocketSend::ping(0).into())
                            .await
                            .unwrap();
                    }
                    WebSocketEvent::Pong { .. } => {
                        panic!("test completed")
                    }

                    _ => {}
                };
            });
        }
    }

    #[tokio::test]
    pub async fn test_authenticate() {
        let (mut read, write) = WebSocket::default().dual_async().await.unwrap();

        if let Ok(ResponseLogin::Success(Session { token, .. })) =
            Client::session_login_smart("EMAIL", "PASSWORD", Some(MFAResponse::totp("CODE")), None)
                .await
        {
            let _client = Client::from_token(&token, "",false).unwrap();
            write
                .write()
                .await
                .send(WebSocketSend::Authenticate { token }.into())
                .await
                .unwrap();
        }

        while let Some(item) = read.next().await {
            if let WebSocketEvent::Authenticated = item {
                println!("SUCCESS")
            }
        }
    }
}
