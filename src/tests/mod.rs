pub mod bots;
pub mod channels;
pub mod servers;
pub mod users;
pub mod websocket;

use reywen_http::Delta;

use crate::client::Client;

pub const SERVER: &str = "01H321YNJZZMF1SYPEY4S9B0R0";
pub const USER: &str = "01FSRTTGJC1XJ6ZEQJMSX8Q96C";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
pub const CHANNEL: &str = "01H3M0YTNP37HKM3MDJSKN7QJ2";
pub const BOT: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const GROUP: &str = "01H482PCMF0Q2H77MTKXPKXQ1B";
pub const USER_NOTSELF: &str = "01H2PWB11T4DY3E76Y8PHFT3EX";
// enter values here for testing

pub fn tester_bot() -> Client {
    test_client(true)
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
    println!("{:#?}", auth);
    let mut client = Client::new();
    let http = Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header(&auth.0, &auth.1)
        .unwrap()
        .set_timeout(10);

    client.http = http;

    client
}
