use super::{
    utils::unix_date_formatting,
    ForceReply,
    InlineKeyboardMarkup,
    Message,
    ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
    User,
};
use crate::api::types::UpdateType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents an incoming callback query from a callback button in
/// an [inline keyboard][kb]. If the button that originated the query was
/// attached to a message sent by the bot, the field message will be present. If
/// the button was attached to a message sent via the bot (in [inline
/// mode][inline]), the field `inline_message_id` will be present. Exactly one
/// of the fields data or `game_short_name` will be present.
///
/// [kb]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
/// [inline]: https://core.telegram.org/bots/api#inline-mode
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query.
    /// Note that message content and message date will not be available if the
    /// message is too old
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot in inline mode, that
    /// originated the query.
    pub inline_message_id: Option<Message>,
    /// Global identifier, uniquely corresponding to the chat to which the
    /// message with the callback button was sent. Useful for high scores in [games](https://core.telegram.org/bots/api#games).
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that a bad client can
    /// send arbitrary data in this field.
    pub data: Option<String>,
    /// Short name of a [`Game`] to be returned, serves as the unique identifier
    /// for the game
    ///
    /// [`Game`]: ../model/struct.Game.html
    pub game_short_name: Option<String>,
}

/// A bot command
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BotCommand {
    /// the command name, for example "ping" for the command "/ping"
    pub command: String,
    /// the description of the command to display in telegram
    pub description: String,
}

/// The Bot API supports basic formatting for messages.
/// You can use bold, italic, underlined and strikethrough text, as well as
/// inline links and pre-formatted code in your bots' messages. Telegram clients
/// will render them accordingly. You can use either markdown-style or
/// HTML-style formatting.
///
/// note: `Markdown` only exists for backwards-compatibility reasons, please use
/// `MarkdownV2`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ParseMode {
    MarkdownV2,
    Markdown,
    HTML,
}

/// An action indicating to a user what they are about to receive
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChatAction {
    /// for a text message
    Typing,
    /// for a photo
    UploadPhoto,
    /// for a video
    RecordVideo,
    /// for a video
    UploadVideo,
    /// for an audio file
    RecordAudio,
    /// for an audio file
    UploadAudio,
    /// for a general file
    UploadDocument,
    /// for a location
    FindLocation,
    /// for a video note
    RecordVideoNote,
    /// for a video note
    UploadVideoNote,
}

/// Enum object for an inline keyboard, custom reply keyboard, instructions to
/// remove reply keyboard or to force a reply from the user.
#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling [`get_file`].
///
/// **Note:** The maximum file size to download is 20 MB
///
/// [`get_file`]: ../api/trait.API.html#method.get_file
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// File size, if known
    pub file_size: Option<i64>,
    /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    /// It is guaranteed that the link will be valid for at least 1 hour. When
    /// the link expires, a new one can be requested by calling getFile
    /// again.
    pub file_path: Option<String>,
}

/// Contains information about the current status of a webhook.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate
    /// checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Unix time for the most recent error that happened when trying to deliver
    /// an update via webhook
    #[serde(with = "unix_date_formatting::optional")]
    pub last_error_date: Option<DateTime<Utc>>,
    /// Error message in human-readable format for the most recent error that
    /// happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery
    pub max_connections: Option<i64>,
    /// A list of update types the bot is subscribed to. Defaults to all update
    /// types
    pub allowed_updates: Option<Vec<UpdateType>>,
    /// Currently used webhook IP address
    pub ip_address: Option<String>,
}
