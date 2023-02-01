use crate::{structs::channel::Channel, client::Web};
#[allow(dead_code)]

pub async fn fetch(domain: &str, channel: &str, token: &str) -> Option<Channel> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/channels/{channel}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "channel_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<Channel>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
#[allow(dead_code)]
pub async fn delete(domain: &str, channel: &str, token: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/channels/{channel}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Web::error(e, "channel_delete");
    };
}
#[allow(dead_code)]
pub async fn edit(domain: &str, channel: &str, token: &str) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/channels/{channel}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Web::error(e, "channel_delete");
    };
}
