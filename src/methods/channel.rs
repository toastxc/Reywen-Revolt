use crate::{client::Web, structs::channel::Channel};
#[allow(dead_code)]

pub async fn fetch(domain: &str, token: &str, header: &str, channel: &str) -> Option<Channel> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/channels/{channel}"))
        .header(header, token)
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
pub async fn delete(domain: &str, token: &str, header: &str, channel: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/channels/{channel}"))
        .header(header, token)
        .send()
        .await
    {
        Web::error(e, "channel_delete");
    };
}
#[allow(dead_code)]
pub async fn edit(domain: &str, token: &str, header: &str, channel: &str) {
    if let Err(e) = reqwest::Client::new()
        .patch(format!("https://{domain}/channels/{channel}"))
        .header(header, token)
        .send()
        .await
    {
        Web::error(e, "channel_delete");
    };
}
