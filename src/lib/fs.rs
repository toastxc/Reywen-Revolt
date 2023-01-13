// library for interacting with the filesystem

use crate::structs::auth::Auth;
use serde_json::Result;
use std::fs::File;
use std::io::Read;

use super::websocket::RWebsocket;

// import and deserialize message.conf
pub fn fs_to_str(target: &str) -> Result<String> {
    let mut file = File::open(target).expect(&format!("could not open {target}"));

    let mut out = String::new();
    file.read_to_string(&mut out)
        .expect("could not read {target}");

    Ok(out)
}
pub fn conf_init() -> Result<Auth> {
    let mut config_json = File::open("config/reywen.json").expect("File not found: reywen.json");

    let mut data_str = String::new();

    config_json
        .read_to_string(&mut data_str)
        .expect("Error while reading file");

    let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

    Ok(conf)
}

pub fn ws_init() -> Result<RWebsocket> {
    let str = fs_to_str("config/reywen.json")?;
    let conf: RWebsocket = serde_json::from_str(&str)?;
    Ok(conf)
}
