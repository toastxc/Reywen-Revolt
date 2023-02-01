use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    client::Web,
    structs::{
        channel::Channel,
        server::{Category, FieldsServer, Server, SystemMessageChannels},
    },
};

/// # Server Data
#[derive(Validate, Deserialize, Serialize)]
pub struct DataCreateServer {
    /// Server name
    #[validate(length(min = 1, max = 32))]
    name: String,
    /// Server description
    #[validate(length(min = 0, max = 1024))]
    description: Option<String>,
    /// Whether this server is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}

/// # Create Server Response
#[derive(Validate, Serialize, Deserialize)]
pub struct CreateServerResponse {
    /// Server object
    server: Server,
    /// Default channels
    channels: Vec<Channel>,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct OptionsServerDelete {
    /// Whether to not send a leave message
    leave_silently: Option<bool>,
}

/// # Server Data
#[derive(Validate, Serialize, Deserialize)]
pub struct DataEditServer {
    /// Server name
    #[validate(length(min = 1, max = 32))]
    name: Option<String>,
    /// Server description
    #[validate(length(min = 0, max = 1024))]
    description: Option<String>,

    /// Attachment Id for icon
    icon: Option<String>,
    /// Attachment Id for banner
    banner: Option<String>,

    /// Category structure for server
    #[validate]
    categories: Option<Vec<Category>>,
    /// System message configuration
    system_messages: Option<SystemMessageChannels>,

    // Whether this server is age-restricted
    // nsfw: Option<bool>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    analytics: Option<bool>,

    /// Fields to remove from server object
    #[validate(length(min = 1))]
    remove: Option<Vec<FieldsServer>>,
}
// ################################## SERVER INFORMATION ##################################

#[allow(dead_code)]
pub async fn create(
    domain: &str,
    server: DataCreateServer,
    token: &str,
) -> Option<CreateServerResponse> {
    match reqwest::Client::new()
        .post(format!("https://{domain}/servers/create"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&server).unwrap())
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "server_create");
            None
        }
        Ok(a) => match serde_json::from_str::<CreateServerResponse>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
#[allow(dead_code)]
pub async fn fetch(domain: &str, server: &str, token: &str) -> Option<Server> {
    match reqwest::Client::new()
        .get(format!("https://{domain}/servers/{server}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Err(http_err) => {
            Web::error(http_err, "server_fetch");
            None
        }
        Ok(a) => match serde_json::from_str::<Server>(&a.text().await.unwrap()) {
            Err(_) => None,
            Ok(a) => Some(a),
        },
    }
}
#[allow(dead_code)]
pub async fn leave(domain: &str, server: &str, token: &str) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/server/{server}"))
        .header("x-bot-token", token)
        .send()
        .await
    {
        Web::error(e, "server_delete");
    };
}
#[allow(dead_code)]
pub async fn edit(domain: &str, server: &str, token: &str, server_edit: DataEditServer) {
    if let Err(e) = reqwest::Client::new()
        .delete(format!("https://{domain}/server/{server}"))
        .header("x-bot-token", token)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&server_edit).unwrap())
        .send()
        .await
    {
        Web::error(e, "sever_delete");
    };
}