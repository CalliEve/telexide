use super::handlers::CommandHandlerFunc;
use crate::{model::BotCommand, utils::result::Error};

#[derive(Clone)]
pub enum CommandTypes {
    Default(CommandHandlerFunc),
}

#[derive(Clone)]
pub struct CommandOptions {
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Clone)]
pub struct TelegramCommand {
    pub options: &'static CommandOptions,
    pub command: CommandTypes,
}

impl TelegramCommand {
    pub fn get_bot_command(&self) -> BotCommand {
        BotCommand {
            command: self.options.name.to_owned(),
            description: self.options.description.to_owned(),
        }
    }
}

/// The error to be returned from a command.
///
/// It can be formed from anything implementing [`std::fmt::Display`], but won't
/// contain more data than a String
#[derive(Debug, Clone)]
pub struct CommandError(pub String);

impl<T: std::fmt::Display> From<T> for CommandError {
    #[inline]
    fn from(d: T) -> Self {
        CommandError(d.to_string())
    }
}

/// A type alias for an std Result with the [`CommandError`] as the Err and ()
/// as the Ok
pub type CommandResult = ::std::result::Result<(), CommandError>;

impl From<CommandError> for Error {
    #[inline]
    fn from(d: CommandError) -> Self {
        Error::Command(d)
    }
}
