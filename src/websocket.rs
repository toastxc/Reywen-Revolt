// reywen
use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{error::Error, Message},
    MaybeTlsStream, WebSocketStream,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Websocket {
    pub token: String,
    pub format: String,
    #[serde(rename = "websocket_domain")]
    pub domain: String,
}

impl Websocket {
    pub fn from_token(token: &str) -> Self {
        Websocket {
            token: String::from(token),
            format: String::from("json"),
            domain: String::from("ws.revolt.chat"),
        }
    }

    pub async fn generate(self) -> impl Stream<Item = Result<Message, Error>> {
        let url = format!(
            "wss://{}/?version=1format={}&token={}",
            self.domain, self.format, self.token
        );

        let ws = connect_async(url)
            .await
            .expect("Failed to connect to websocket!");
        let (stream, _response) = ws;

        let (write, read) = stream.split();
        tokio::spawn(ping_server(Mutex::new(write)));

        read
    }
}

async fn ping_server(write: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

        write
            .lock()
            .await
            .send(Message::Text(String::from(
                "{\n    \"type\": \"Ping\",\n    \"data\": 0\n}",
            )))
            .await
            .ok();
    }
}

pub fn from_ws(
    message: Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>,
) -> String {
    match message {
        Ok(a) => a
            .into_text()
            .expect("websocket message failed (report to developer)"),
        Err(e) => {
            println!("Websocket Error!\n{}", e);
            String::new()
        }
    }
}
