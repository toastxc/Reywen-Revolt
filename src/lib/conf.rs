use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MainConf {
    pub message: MessageConf,
    pub auth: Auth,
    pub bridge: BrConf,
    pub shell: ShellConf,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellConf {
    pub enabled: bool,
    pub whitelist_sudoers: bool,
    pub enable_sudo: bool,
    pub shell_channel: SocConf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocConf {
    pub enabled: bool,
    pub channel: String,
}
