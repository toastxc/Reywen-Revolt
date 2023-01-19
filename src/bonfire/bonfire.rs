// reywen
use crate::quark::bonfire::RWebsocket;
use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{error::Error, Message},
    MaybeTlsStream, WebSocketStream,
};
impl RWebsocket {
    pub fn new(ws: RWebsocket) -> Self {
        ws
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

        tokio::spawn(ping_server(write));

        read
    }
}

async fn ping_server(mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
    let wait = tokio::time::Duration::from_secs(30);

    loop {
        tokio::time::sleep(wait).await;

        write
            .send(Message::Text(String::from(
                "{\n    \"type\": \"Ping\",\n    \"data\": 0\n}",
            )))
            .await
            .unwrap();
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
