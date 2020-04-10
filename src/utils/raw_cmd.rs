use crate::framework::types::{CommandOptions, CommandTypes, TelegramCommand};
use crate::client::event_handlers::{MessageHandlerFunc, InlineResultHandlerFunc, InlineQueryHandlerFunc};

#[derive(Clone)]
pub enum RawCommandTypes {
    Default(MessageHandlerFunc),
    Inline(InlineQueryHandlerFunc),
    InlineResult(InlineResultHandlerFunc),
}

pub struct RawTelegramCommand {
    pub options: &'static CommandOptions,
    pub command: RawCommandTypes,
}

impl From<&RawTelegramCommand> for TelegramCommand {
    fn from(raw: &RawTelegramCommand) -> Self {
        let opts = raw.options;
        let cmd = match raw.command.clone() {
            RawCommandTypes::Default(c) => CommandTypes::Default(c.into()),
            RawCommandTypes::InlineResult(c) => CommandTypes::InlineResult(c.into()),
            RawCommandTypes::Inline(c) => CommandTypes::Inline(c.into())
        };

        Self {
            options: opts,
            command: cmd,
        }
    }
}
