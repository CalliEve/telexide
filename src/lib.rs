//! ## Telexide is a rust library for the telegram API
//!
//! View the [examples] to see practical examples of how to use the library.
//!
//! Use the [`ClientBuilder`] to easily create a [`Client`] object to your
//! preferences and register commands with the [`create_framework`] macro and/or
//! register your own update handlers, before running [`Client::start`] to start
//! your bot. All of this is designed to be highly customisable. For further
//! information, please see [client's module-level documentation][client].
//!
//! Telegram also has their own [API docs for bots][tg docs], which should
//! always be referenced in case of discrepancies between the working of this
//! library and what is expected.
//!
//! # Resources
//!  - [Examples][examples]
//!  - [Github Repository]
//!  - [crates.io]
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! telexide = "0.1"
//! ```
//!
//! [examples]: https://github.com/Baev1/telexide/tree/master/examples
//! [Github Repository]: https://github.com/Baev1/telexide
//! [crates.io]: https://crates.io/crates/telexide
//! [tg docs]: https://core.telegram.org/bots/api
//! [client]: client/index.html
//! [`ClientBuilder`]: client/struct.ClientBuilder.html
//! [`Client`]: client/struct.Client.html
//! [`Client::start`]: client/struct.Client.html#method.start

#![allow(where_clauses_object_safety)] // TODO: make this unnecessary by refactoring the API trait?
#![warn(clippy::pedantic)]
#![allow(
    dead_code,
    clippy::module_inception,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::wildcard_imports
)]

pub mod api;
pub mod client;
pub mod framework;
pub mod model;
mod utils;

/// Macros for using the framework and helping with adding listeners
pub mod macros {
    pub use super::create_framework;
    pub use subscription_macros::{command, prepare_listener};
}

pub use client::Client;
pub use utils::result::{Error, Result};

pub mod prelude {
    //! A default set of exports which can be helpful to use.
    //!
    //! note that `TelexideError` is a re-export of [`telexide::Error`] under a
    //! different name to remove likely ambiguity with other crate error
    //! enums.
    //!
    //! ## Examples
    //!
    //! Import all of the exports:
    //!
    //! ```rust
    //! use telexide::prelude::*;
    //! ```
    //!
    //! [`telexide::Error`]: ../enum.Error.html

    pub use super::{
        client::{Client, ClientBuilder, Context},
        create_framework,
        model::{Message, Update},
        Error as TelexideError,
    };
    pub use subscription_macros::{command, prepare_listener};
}

#[doc(hidden)]
pub use utils::raw_cmd;
