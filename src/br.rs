use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrConf {
    pub enabled: bool,
    pub channel_1: String,
    pub channel_2: String,
}
