//! The framework provides a customizable way to manage your bots commands

pub(crate) mod framework;

// made public for the procedural macros to use
#[doc(hidden)]
pub mod handlers;
#[doc(hidden)]
pub mod types;

pub use framework::Framework;
pub use types::{CommandError, CommandResult};
