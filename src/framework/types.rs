use crate::{
    client::{InlineQueryHandler, InlineResultHandler, MessageHandler},
    model::BotCommand,
};

#[derive(Clone)]
pub enum CommandTypes {
    Default(MessageHandler),
    Inline(InlineQueryHandler),
    InlineResult(InlineResultHandler),
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
