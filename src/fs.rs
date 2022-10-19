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

pub fn conf_init() -> Result<MainConf> {

  
    let mut config_json = File::open("reywen.json")
        .expect("File not found: reywen.json");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: MainConf = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}
