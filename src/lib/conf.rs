use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainConf {
    pub message: MessageConf,
    pub auth: Auth,
    pub bridge: BrConf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageConf {
    
    pub message_enabled: bool
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {

    pub token: String,
    pub bot_id: String,
    pub sudoers: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrConf {
    
    pub bridge_enabled: bool,
    pub channel_1: String,
    pub channel_2: String,
}

