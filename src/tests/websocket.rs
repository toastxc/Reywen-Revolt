use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use futures_util::{SinkExt, StreamExt};

    use crate::{
        tests::websocket::time_helper,
        websocket::{
            data::{WebSocketEvent, WebSocketSend},
            WebSocket,
        },
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
                println!("yipeee!");
                return;
            }
        }
    }

    #[tokio::test]
    pub async fn ws_full_test() {
        let ws = WebSocket::from_token(
            "",
        )
        .dual_connection()
        .await;

        let (mut write, mut read) = ws;

        while let Some(item) = read.next().await {
            // if the event is a message
            match item {
                WebSocketEvent::Message { .. } => {
                    write
                        .send(WebSocketSend::ping(time_helper()).into())
                        .await
                        .ok();
                }
                WebSocketEvent::Pong { data } => {
                    let ping = time_helper() - data;
                    println!("{}", ping);
                    return;
                }

                _ => {}
            };
        }
    }
}

pub fn time_helper() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize
}
