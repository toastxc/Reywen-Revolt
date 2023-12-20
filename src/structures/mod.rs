pub mod authentication;
pub mod channels;
pub mod media;
pub mod permissions;
pub mod server;
pub mod users;
pub mod emoji;

#[macro_export]
macro_rules! impl_to_vec {
    ($data:ident) => {
        impl From<&$data> for Option<Vec<u8>> {
            fn from(value: &$data) -> Self {
                Some(serde_json::to_string(value).unwrap().into_bytes())
            }
        }
    };
}
#[macro_export]
macro_rules! str {
    ($name:ident) => {
        $name: impl Into<String>
    };
}
