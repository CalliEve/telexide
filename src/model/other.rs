use super::{Message, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<Message>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}
