pub mod client;
pub mod structures;
pub mod tests;
pub mod websocket;

#[cfg(all(feature = "standard", feature = "wasm"))]
compile_error!("Multiple HTTP engines cannot be used together");

#[cfg(feature = "standard")]
pub mod reywen_http {
    pub mod utils {
        pub use reywen_http::utils::*;
        pub fn if_false(input: &bool) -> bool {
            !input.to_owned()
        }
    }
    pub use reywen_http::engines::hyper::Hyper as Delta;
    pub mod results {
        pub use reywen_http::engines::hyper::results::Error as DeltaError;
    }
    pub mod driver {
        pub use reywen_http::engines::hyper;
        pub use reywen_http::engines::hyper::Method;
    }
}

#[cfg(feature = "wasm")]
pub mod reywen_http {
    pub mod utils {
        pub use reywen_http::utils::*;
        pub fn if_false(input: &bool) -> bool {
            !input.to_owned()
        }
    }
    pub use reywen_http::engines::reqwasm::Reqwasm as Delta;
    pub mod results {
        pub use reywen_http::engines::reqwasm::results::Error as DeltaError;
    }
    pub mod driver {
        pub use reywen_http::engines::reqwasm;
        pub use reywen_http::engines::reqwasm::Method;
    }
}
