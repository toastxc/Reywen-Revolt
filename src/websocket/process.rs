use super::{data::WebSocketEvent, PartialWSConf, WebSocket};
use crate::{
    reywen_http::utils::struct_to_url, websocket::data::WebSocketSend, websocket::error::Error,
};
use futures_util::{stream::SplitSink, SinkExt, Stream, StreamExt};
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

impl WebSocket {
    pub async fn dual_async(
        &self,
    ) -> Result<
        (
            Pin<Box<dyn Stream<Item = WebSocketEvent>>>,
            Arc<RwLock<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        ),
        Error,
    > {
        let url = format!(
            "{}{}{}",
            self.domain.clone(),
            {
                if self.domain.clone().ends_with('/') {
                    "/"
                } else {
                    ""
                }
            },
            struct_to_url(Into::<PartialWSConf>::into(self.to_owned()))
        )
        .replace('\"', "");

        let url = url::Url::parse(&url)?;

        let (ws_stream, _) = connect_async(url).await?;
        let (write, read) = ws_stream.split();

        let write = Arc::new(RwLock::new(write));

        tokio::spawn(ws_maintain(Arc::clone(&write)));

        Ok((
            Box::pin(read.filter_map(|result| async {
                result
                    .map(|a| serde_json::from_slice::<WebSocketEvent>(&a.into_data()).ok())
                    .ok()
                    .flatten()
            })) as Pin<Box<dyn Stream<Item = WebSocketEvent>>>,
            write,
        ))
    }
}

async fn ws_maintain(
    write: Arc<RwLock<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
) {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        write
            .write()
            .await
            .send(WebSocketSend::Ping { data: 0 }.into())
            .await
            .ok();
    }
}
