// library for interacting with the filesystem

use serde_json::{Result};
use crate::lib::{
    conf::Auth
};


use std::{
    io::Read,
    fs::File,
};

// import and deserialize message.conf


pub fn fs_str(target: &str) -> Result<String> {


    let mut file = File::open(target)
        .expect("could not open {target}");

    let mut out = String::new();
    file.read_to_string(&mut out)
        .expect("could not read {target}");




    Ok(out)


}
/*
pub fn str_fs(target: &str, payload: &str) {

   let mut file = File::create(target)
        .expect("could not open {target}");


   file.write_all(payload.as_bytes()).unwrap();


}
*/
pub fn conf_init() -> Result<Auth> {

  
    let mut config_json = File::open("reywen.json")
        .expect("File not found: reywen.json");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}
