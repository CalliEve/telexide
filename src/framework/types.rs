use crate::{
    model::BotCommand,
    utils::result::Error
};
use super::handlers::CommandHandlerFunc;

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

#[derive(Debug, Clone)]
pub struct CommandError(pub String);

impl<T: std::fmt::Display> From<T> for CommandError {
    #[inline]
    fn from(d: T) -> Self {
        CommandError(d.to_string())
    }
}

pub type CommandResult = ::std::result::Result<(), CommandError>;

impl From<CommandError> for Error {
    #[inline]
    fn from(d: CommandError) -> Self {
        Error::Command(d)
    }
}
