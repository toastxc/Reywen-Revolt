/// library for interacting with the filesystem
use std::{fs::File, io::Read};

use serde_json::Result;

use crate::quark::{bonfire::RWebsocket, delta::auth::Auth};

/// generic method for deserilizing files
pub fn fs_to_str(target: &str) -> Result<String> {
    let mut file = File::open(target).unwrap_or_else(|_| panic!("could not open {target}"));

    let mut out = String::new();
    file.read_to_string(&mut out)
        .expect("could not read {target}");

    Ok(out)
}
/// import websocket config
pub fn ws_init() -> Result<RWebsocket> {
    let str = fs_to_str("config/reywen.json")?;
    let conf: RWebsocket = serde_json::from_str(&str)?;
    Ok(conf)
}
/// import auth config
pub fn conf_init() -> Result<Auth> {
    let str = fs_to_str("config/reywen.json")?;
    let conf: Auth = serde_json::from_str(&str)?;
    Ok(conf)
}
