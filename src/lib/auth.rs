use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {

    pub token: String,
    pub bot_id: String,
    pub sudoers: Vec<String>,
}

