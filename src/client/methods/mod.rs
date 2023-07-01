pub mod bot;
pub mod channel;
pub mod group;
pub mod member;
pub mod message;
pub mod permissions;
pub mod server;
pub mod user;

#[macro_export]
macro_rules! json {
    ($data:expr) => {
        Some(&serde_json::to_string(&$data).unwrap_or_default())
    };
}
