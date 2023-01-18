/// # Reywen
/// Reywen features are named are the backend component they connect to
/// - delta - http API
/// - quark - common methods and structures
/// - bonfire - websocket

/// # http methods for reywen! as well as other useful features
pub mod delta {
    pub mod delta;
    pub mod fs;
    pub mod lreywen;
    pub mod mongo;
    pub mod oop;
}
/// # websocket features for reywen
pub mod bonfire {
    pub mod bonfire;
}

/// # structs
/// common datastructures for reywen
pub mod quark {
    pub mod delta {
        pub mod auth;
        pub mod message;
        pub mod user;
    }
    pub mod bonfire;
    pub mod mongo;
}
