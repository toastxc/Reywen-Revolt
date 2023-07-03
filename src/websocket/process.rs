use std::{pin::Pin, sync::Arc};

use futures_util::{
    stream::{SplitSink, SplitStream},
    Stream, StreamExt,
};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, WebSocketStream};

use super::{data::WebSocketEvent, WebSocket};

impl WebSocket {
    pub async fn stream(input: Connection) -> Pin<Box<impl Stream<Item = WebSocketEvent>>> {
        Box::pin({
            (input).filter_map(|result| async {
                match result {
                    Ok(message) => serde_json::from_slice(&message.into_data()).ok(),
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                }
            })
        })
    }

    pub async fn generate(self) -> Connection {
        let url = format!(
            "wss://{}/?version=1format={}&token={}",
            self.domain, self.format, self.token
        );

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        ws_stream
    }

    pub async fn start(self) -> Pin<Box<impl Stream<Item = WebSocketEvent>>> {
        WebSocket::stream(self.generate().await).await
    }

    pub async fn new_stream(input: StreamSplit) -> Pin<Box<impl Stream<Item = WebSocketEvent>>> {
        Box::pin({
            (input).filter_map(|result| async {
                match result {
                    Ok(message) => serde_json::from_slice(&message.into_data()).ok(),
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                }
            })
        })
    }

    pub async fn dual_connection(
        &self,
    ) -> (SinkSplit, Pin<Box<impl Stream<Item = WebSocketEvent>>>) {
        let ws = self.clone().generate().await;

        let (write, read) = ws.split();

        (write, WebSocket::new_stream(read).await)
    }

    /// This is the recommended connection client
    /// 0: a modified SplitStream websocket connection with the WebSocketEvent data type
    /// 1: a modified SplitSink within an Arc<Mutex>, this can locked and have messages sent on it async.
    /// Arc allows for cloning* the type which is useful for multithreading
    pub async fn dual_async(
        &self,
    ) -> (
        Pin<Box<impl Stream<Item = WebSocketEvent>>>,
        Arc<Mutex<SinkSplit>>,
    ) {
        let (write, read) = Self::dual_connection(self).await;

        (read, Arc::new(Mutex::new(write)))
    }
}

type Connection = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

type StreamSplit =
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;

type SinkSplit = SplitSink<
    WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;
