//! The framework provides a customizable way to manage your bots commands

pub(crate) mod framework;

// made public for the procedural macros to use
#[doc(hidden)]
pub mod types;
#[doc(hidden)]
pub mod handlers;

pub use types::{CommandResult, CommandError};
pub use framework::Framework;
