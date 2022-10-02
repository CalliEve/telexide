use serde::{Deserialize, Serialize};

use super::utils::IntegerOrString;

/// A bot command
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BotCommand {
    /// the command name, for example "ping" for the command "/ping"
    pub command: String,
    /// the description of the command to display in telegram
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "all_private_chats")]
    AllPrivateChats,
    #[serde(rename = "all_group_chats")]
    AllGroupChats,
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators,
    #[serde(rename = "chat")]
    Chat { chat_id: IntegerOrString },
    #[serde(rename = "chat_administrators")]
    ChatAdministrators { chat_id: IntegerOrString },
    #[serde(rename = "chat_member")]
    ChatMember {
        chat_id: IntegerOrString,
        user_id: i64,
    },
}
