#![allow(where_clauses_object_safety)]
// #![warn(clippy::pedantic)]
#![allow(dead_code, clippy::module_inception)]

/// The api module provides the API trait and APIClient, providing methods to perform requests to the telegram API
pub mod api;
/// The Client manages your event handlers, in addition it also manages your api connection for you
pub mod client;
/// the framework provides a customizable way to manage your bot's commands
pub mod framework;
/// Mappings of objects received from the API, with some optional helper methods for ease of use.
pub mod model;
mod utils;

pub mod macros {
    pub use super::create_framework;
    pub use subscription_macros::{command, prepare_listener};
}

pub use client::Client;
pub use utils::result::{Result, Error};

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