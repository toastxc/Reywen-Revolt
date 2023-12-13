pub mod bot;
pub mod channel;
pub mod group;
pub mod member;
pub mod message;
pub mod permissions;
pub mod server;
pub mod session;
pub mod user;
/// macro for creating jsons
#[macro_export]
macro_rules! json {
    ($data:expr) => {
        Some(
            serde_json::to_string(&$data)
                .unwrap_or_default()
                .as_bytes()
                .to_owned(),
        )
    };
}

#[macro_export]
macro_rules! ref_str {
    ($data:expr) => {
        &String::from($data)
    };
}

#[macro_export]
macro_rules! opt_str {
    ($data:expr) => {
        Some(String::from($data))
    };
}

/// autovec is a helper for adding an entry to an optional vector, since this code is quite big and is repeated many times it has
/// been given its own function
// pub fn opt_vec_add<T: Clone>(input: &Option<Vec<T>>, new: &T) -> Option<Vec<T>> {
//     Some(input.clone().map_or(vec![new.to_owned()], |mut origin| {
//         origin.push(new.to_owned());
//         origin
//     }))
// }
pub fn opt_vec_add<T: Clone>(input: &mut Option<Vec<T>>, new: T) {
    input.get_or_insert_with(Vec::new).push(new);
}

// grabs original value and appends it to an optional vector
pub fn origin<T: Clone + Default>(input: &mut Option<Vec<T>>, new: Vec<T>) {
    input.get_or_insert_with(Vec::new).extend(new);
}
