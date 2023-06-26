#[cfg(test)]
mod tests {
    use futures_util::StreamExt;

    use crate::websocket::{data::WebSocketEvent, Websocket};

    // test of reywenv3 websocket owo
    #[tokio::test]
    pub async fn ws_stream_test() {
        let input = Websocket::from_token("");
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
}
