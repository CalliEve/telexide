//! Telexide is a rust library for the telegram API
//!
//! View the [examples] to see practical examples of how to use the library.
//!
//! Use the [`ClientBuilder`] to easily create a [`Client`] object to your
//! preferences and register commands with the [`create_framework`] macro and/or
//! register your own update handlers, before running [`Client::start`] to start
//! your bot. All of this is designed to be highly customisable. For further
//! information about the client, please see the [client's module-level
//! documentation][client].
//!
//! API calls are easy to make using the [`APIClient`] and the api data models,
//! or create and use your own api client by implementing the [`API`] trait. For
//! further information about the api client, please see the [api's module-level
//! documentation][api].
//!
//! A default command framework is provided using the [`Framework`] object,
//! providing easy handling of incoming [telegram bot commands][tg_commands]
//! sent by users of your bot. For further information about the framework,
//! please see the [framework's module-level documentation][framework].
//!
//! Telegram also has their own [API docs for bots][tg docs]. Although this
//! documentation will try to be as accurate as possible, if you need to be
//! sure, refer to their docs.
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
//! [examples]: https://github.com/callieve/telexide/tree/master/examples
//! [Github Repository]: https://github.com/callieve/telexide
//! [crates.io]: https://crates.io/crates/telexide
//! [tg docs]: https://core.telegram.org/bots/api
//! [client]: client/index.html
//! [`ClientBuilder`]: client/struct.ClientBuilder.html
//! [`Client`]: client/struct.Client.html
//! [`Client::start`]: client/struct.Client.html#method.start
//! [`APIClient`]: api/struct.APIClient.html
//! [`API`]: api/trait.API.html
//! [api]: api/index.html
//! [tg_commands]: https://core.telegram.org/bots#commands
//! [`Framework`]: framework/struct.Framework.html
//! [framework]: framework/index.html

#![warn(clippy::pedantic)]
#![allow(
    dead_code,
    clippy::module_inception,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::wildcard_imports,
    clippy::too_many_lines
)]

pub mod api;
pub mod client;
pub mod framework;
pub mod model;
mod utils;

/// Macros for using the framework and helping with adding listeners
pub mod macros {
    pub use super::create_framework;
    pub use telexide_proc_macros::{command, prepare_listener};
}

pub use client::Client;
pub use utils::result::{Error, TelegramError, Result};

pub mod prelude {
    //! A default set of exports which can be helpful to use.
    //!
    //! note that [`TelexideError`] is a re-export of [`telexide::Error`] under
    //! a different name to remove likely ambiguity with other crate error
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
    //! [`TelexideError`]: ../enum.Error.html

    pub use super::{
        client::{Client, ClientBuilder, Context},
        create_framework,
        framework::CommandResult,
        model::{Message, Update},
        Error as TelexideError,
    };
    pub use telexide_proc_macros::{command, prepare_listener};
}

#[doc(hidden)]
#[allow(unused_imports)]
pub use paste::expr as paste_expr;
