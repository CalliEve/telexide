use crate::model::{utils::IntegerOrString, ReplyMarkup};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`send_game`]
///
/// [`send_game`]:
/// ../../api/trait.API.html#method.send_game
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread (topic) of the forum;
    /// for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Short name of the game, serves as the unique identifier for the game.
    /// Set up your games via Botfather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`set_game_score`]
///
/// [`set_game_score`]:
/// ../../api/trait.API.html#method.set_game_score
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score
    pub score: i64,
    /// Pass True, if the high score is allowed to decrease.
    /// This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to
    /// include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// struct for holding data needed to call
/// [`get_game_high_scores`]
///
/// [`get_game_high_scores`]:
/// ../../api/trait.API.html#method.get_game_high_scores
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
