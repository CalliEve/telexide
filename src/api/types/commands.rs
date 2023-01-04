use crate::{
    framework::types::TelegramCommand,
    model::{BotCommand, BotCommandScope},
};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`set_my_commands`]
///
/// [`set_my_commands`]:
/// ../../api/trait.API.html#method.set_my_commands
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyCommands {
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
}

impl From<Vec<BotCommand>> for SetMyCommands {
    fn from(commands: Vec<BotCommand>) -> Self {
        Self {
            commands,
            language_code: None,
            scope: None,
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
            language_code: None,
            scope: None,
        }
    }
}

/// struct for holding data needed to call
/// [`get_my_commands`]
///
/// [`get_my_commands`]:
/// ../../api/trait.API.html#method.get_my_commands
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
}

/// struct for holding data needed to call
/// [`delete_my_commands`]
///
/// [`delete_my_commands`]:
/// ../../api/trait.API.html#method.delete_my_commands
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
}
