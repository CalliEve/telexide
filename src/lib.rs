#![allow(where_clauses_object_safety)]

pub mod api;
pub mod client;
pub mod framework;
pub mod model;
mod utils;

pub mod macros {
    pub use super::create_framework;
    pub use subscription_macros::{command, prepare_listener};
}

pub use utils::result::Result;

pub mod prelude {
    pub use super::{
        client::{Client, ClientBuilder, Context},
        create_framework,
        model::{Message, Update},
    };
    pub use subscription_macros::{command, prepare_listener};
}

#[doc(hidden)]
pub use utils::raw_cmd;