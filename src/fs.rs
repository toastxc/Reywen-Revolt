// library for interacting with the filesystem

use serde_json::{Result};
use crate::lib::{
    conf::MainConf
};


use std::{
    io::Read,
    fs::File,
};

// import and deserialize message.conf

pub fn Conf_init() -> Result<MainConf> {

  
    let mut config_json = File::open("reywen.json")
        .expect("File not found: reywen.json");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: MainConf = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}

/*
// import and deserialize auth.conf
pub fn auth_init() -> Result<Auth> {

    let mut config_json = File::open("auth.json")
        .expect("File not found");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}


// import and deserialize bridge.json
pub fn bridge_init() -> Result<BrConf> {

    let mut config_json = File::open("bridge.json")
        .expect("bridge config file not found");

    let mut brconf_str = String::new();
    config_json.read_to_string(&mut brconf_str)
        .expect("Error while reading file");


    let config: BrConf = serde_json::from_str(&brconf_str).expect("failed to interpret brconf");

    Ok(config)
}
*/
