#[cfg(test)]
mod tests {
    use futures_util::{SinkExt, StreamExt};

    use crate::tests::websocket::ping_data;
    use crate::{
        client::Client,
        structures::channels::message::Message,
        websocket::{data::WebSocketEvent, Websocket},
    };

    use crate::client::methods::message::DataMessageSend;
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
                println!("yipeee!")
            }
        }
    }
    #[tokio::test]
    pub async fn ws_stream_test_2() {
        let client = Client::from_token("", true).unwrap();

        let ws = client.websocket.clone().generate().await;

        let (mut write, read) = ws.split();

        let mut mod_ws = Websocket::new_stream(read).await;

        while let Some(message) = mod_ws.next().await {
            match message {
                WebSocketEvent::Message {
                    message:
                        Message {
                            content: Some(message),
                            channel,
                            ..
                        },
                } => {
                    if message.as_str() == ";\\ send ping" {
                        println!("message received");
                        write.send(ping_data()).await.ok();
                    };
                }
                WebSocketEvent::Pong { .. } => {
                    client
                        .message_send(
                            "01H321YNJZXSJFJ8TKHZ1P5SGX",
                            &DataMessageSend::new().set_content("womp"),
                        )
                        .await
                        .ok();
                }

                _ => {}
            }
        }
    }
}
