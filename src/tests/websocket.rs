#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures_util::{SinkExt, StreamExt};

    use crate::websocket::{
        data::{WebSocketEvent, WebSocketSend},
        WebSocket,
    };

    // test of reywenv3 websocket owo
    #[tokio::test]
    pub async fn ws_stream_test() {
        let input = WebSocket::from_token("");
        // generate a websocket connection AND convert types
        let mut ws = input.to_owned().start().await;

        // for every message
        while let Some(item) = ws.next().await {
            // if the event is a message
            if let WebSocketEvent::Message { .. } = item {
                return;
            }
        }
    }

    #[tokio::test]

    pub async fn ws_full_test() {
        let ws = WebSocket::from_token("").dual_connection().await;

        let (mut write, mut read) = ws;

        while let Some(item) = read.next().await {
            // if the event is a message
            match item {
                WebSocketEvent::Message { .. } => {
                    write.send(WebSocketSend::ping(0).into()).await.ok();
                }
                WebSocketEvent::Pong { .. } => {
                    return;
                }

                _ => {}
            };
        }
    }

    #[tokio::test]
    pub async fn ws_test_async() {
        let ws = WebSocket::from_token(
            "kRy0tMo6Mkc2pPeiRKN3g-phqVnUqk88ME6XaAlztZsAZkTd3tVZBFKyq88ZLi6j",
        );

        let (mut read, write) = ws.dual_async().await;

        while let Some(item) = read.next().await {
            let write = Arc::clone(&write);

            // if the event is a message
            tokio::spawn(async move {
                match item {
                    WebSocketEvent::Message { .. } => {
                        write
                            .lock()
                            .await
                            .send(WebSocketSend::ping(0).into())
                            .await
                            .unwrap();
                    }
                    WebSocketEvent::Pong { .. } => {
                        return;
                    }

                    _ => {}
                };
            });
        }
    }
}
