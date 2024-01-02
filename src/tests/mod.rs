pub mod bots;
pub mod channels;
pub mod experiments;
pub mod servers;
pub mod session;
pub mod users;
pub mod websocket;
use crate::client::Client;
use dotenv::*;
use std::env;
pub mod emoji;
pub const SERVER: &str = "01H321YNJZZMF1SYPEY4S9B0R0";
pub const USER: &str = "01FSRTTGJC1XJ6ZEQJMSX8Q96C";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
pub const CHANNEL: &str = "01H3M0YTNP37HKM3MDJSKN7QJ2";
pub const BOT: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const GROUP: &str = "01H482PCMF0Q2H77MTKXPKXQ1B";
pub const USER_NOTSELF: &str = "01H2PWB11T4DY3E76Y8PHFT3EX";
// enter values here for testing

pub fn tester_bot() -> Client {
    dotenv().ok();

    Client::from_token(
        env::var("DEVEL_BOT_TOKEN").unwrap(),
        env::var("DEVEL_BOT_ID").unwrap(),
        true,
    )
    .unwrap()
}

pub fn tester_user() -> Client {
    dotenv().ok();
    Client::from_token(
        env::var("DEVEL_USER_TOKEN").unwrap(),
        env::var("DEVEL_USER_ID").unwrap(),
        false,
    )
    .unwrap()
}
