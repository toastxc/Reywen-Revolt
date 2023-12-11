use super::{data::WebSocketEvent, PartialWSConf, WebSocket};
use crate::{reywen_http::utils::struct_to_url, websocket::error::Error};
use futures_util::{stream::SplitSink, Stream, StreamExt};
use std::{pin::Pin, sync::Arc};
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, WebSocketStream};

impl WebSocket {
    pub async fn dual_async(
        &self,
    ) -> Result<
        (
            Pin<Box<dyn Stream<Item = WebSocketEvent>>>,
            Arc<RwLock<SinkSplit>>,
        ),
        Error,
    > {
        let (ws_stream, _) = connect_async(
            format!(
                "wss://{}/{}",
                self.domain.clone(),
                struct_to_url(Into::<PartialWSConf>::into(self.to_owned()))
            )
            .replace('\"', ""),
        )
        .await?;
        let (write, read) = ws_stream.split();

        Ok((
            Box::pin((read).filter_map(|result| async {
                result
                    .map(|a| serde_json::from_slice(&a.into_data()).unwrap())
                    .ok()
            })) as Pin<Box<dyn Stream<Item = WebSocketEvent>>>,
            Arc::new(RwLock::new(write)),
        ))
    }
}

type SinkSplit = SplitSink<
    WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Message,
>;
