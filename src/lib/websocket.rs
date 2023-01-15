use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RWebsocket {
    token: String,
    format: String,
    #[serde(rename = "websocket_domain")]
    domain: String,
}

impl RWebsocket {
    pub fn new(ws: RWebsocket) -> Self {
        ws
    }

    pub async fn generate(
        self,
    ) -> futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    > {
        let url = format!(
            "wss://{}/?version=1format={}&&token={}",
            self.domain, self.format, self.token
        );
        let ws = connect_async(url.clone())
            .await
            .expect("Failed to connect to websocket!");
        let (stream, _response) = ws;

        let (_, read) = stream.split();
        read
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
