pub mod bots;
pub mod channels;
pub mod servers;
pub mod users;
pub mod websocket;

use reywen_http::driver::Delta;

use crate::client::Client;

pub const SERVER: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
pub const USER: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
pub const CHANNEL: &str = "01GKWVWGHNBNCFPC9Q7CRDHBVZ";
pub const GROUP: &str = "01GYM0JBNKWRJYX56F9GYABS4R";
pub const BOT: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
// enter values here for testing

pub fn tester_bot() -> Client {
    test_client(false)
}

pub fn tester_user() -> Client {
    test_client(false)
}

fn bot_or(is_bot: bool) -> (String, String) {
    if is_bot {
        (
            String::from("x-bot-token"),
            include_str!("bot-token.txt").to_string(),
        )
    } else {
        (
            String::from("x-session-token"),
            include_str!("self-token.txt").to_string(),
        )
    }
}

pub fn test_client(is_bot: bool) -> Client {
    let auth = bot_or(is_bot);
    let mut client = Client::new();
    let http = Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header(&auth.0, &auth.1)
        .unwrap()
        .set_timeout(10);

    client.http = http;

    client
}
