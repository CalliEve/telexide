use crate::{framework::types::TelegramCommand, model::BotCommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommand>,
}

impl From<Vec<BotCommand>> for SetMyCommands {
    fn from(commands: Vec<BotCommand>) -> Self {
        Self {
            commands,
        }
    }
}

impl From<&Vec<TelegramCommand>> for SetMyCommands {
    fn from(commands: &Vec<TelegramCommand>) -> Self {
        Self {
            commands: commands.iter().map(|c| c.get_bot_command()).collect(),
        }
    }
}
