/// # http methods
pub mod delta {
    /// http
    pub mod delta;
    /// filesystem abstraction
    pub mod fs;
    /// http abstraction
    pub mod lreywen;
    /// abstractions for mongodb
    pub mod mongo;
    /// builder patterns for http
    pub mod oop;
}
/// # websocket
pub mod bonfire {
    pub mod bonfire;
}
/// # common structures for reywen
pub mod quark {
    /// # structs for http
    pub mod delta {
        pub mod auth;
        pub mod message;
        pub mod user;
    }
    /// structs for websocket
    pub mod bonfire;
    /// structs for mongodb
    pub mod mongo;
}
