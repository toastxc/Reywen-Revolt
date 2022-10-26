// dependancies 

use crate::{Auth, RMessage, lib::message::*};
use crate::fs_str;
use serde::{Serialize, Deserialize};


// config struct
// this optional struct adds configurable paramaters that are hot changeable, config files are
// jsons and usually stored in config/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mogus {
    pub enable: bool,
    pub id: String,
    pub channel: String
}


// plugin main is responsible for getting details and activiating functions based on conditions
pub async fn plugin_main(auth: Auth, input_message: RMessage) {

    let conf = fs_str("config/plugin.json")
        .expect("failed to read config/plugin.json\n{e}");

    let c: Mogus = serde_json::from_str(&conf.unwrap())
        .expect("Failed to deser plugin.json");



    // if the config channel matches the channel of the message received AND 
    // if the plugin is enabled, send ID
    if c.enabled == true && c.channel == input_message.channel {

        send(id).await

    };


    // there are many other possible features of reywen plugins but this is a good simple starting
    // point

}

