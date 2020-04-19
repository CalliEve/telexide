use serde::{Deserialize, Serialize};
use crate::model::{ReplyMarkup};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Short name of the game, serves as the unique identifier for the game.
    /// Set up your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score
    pub score: i64,
    /// Pass True, if the high score is allowed to decrease.
    /// This can be useful when fixing mistakes or banning cheaters
    pub force: bool,
    /// Pass True, if the game message should not be automatically edited to include the current scoreboard
    pub disable_edit_message: bool,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
