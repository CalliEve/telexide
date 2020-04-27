use crate::{framework::types::TelegramCommand, model::BotCommand};
use serde::{Deserialize, Serialize};

/// struct for holding data needed to call
/// [`set_my_commands`]
///
/// [`set_my_commands`]:
/// ../../api/trait.API.html#method.set_my_commands
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
            commands: commands
                .iter()
                .map(TelegramCommand::get_bot_command)
                .collect(),
        }
    }
}
