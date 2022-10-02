use super::InputMedia;
use crate::model::{
    utils::IntegerOrString, InlineKeyboardMarkup, Message, MessageEntity, ParseMode,
};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`edit_message_text`]
///
/// [`edit_message_text`]:
/// ../../api/trait.API.html#method.edit_message_text
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageText {
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing.
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in message text, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    fn from_message(message: &Message, new_text: &str) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            text: new_text.to_owned(),
            inline_message_id: None,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_caption`]
///
/// [`edit_message_caption`]:
/// ../../api/trait.API.html#method.edit_message_caption
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageCaption {
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            caption: None,
            inline_message_id: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_media`]
///
/// [`edit_message_media`]:
/// ../../api/trait.API.html#method.edit_message_media
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageMedia {
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// new media content of the message.
    pub media: InputMedia,
    /// Inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageMedia {
    fn from_message(message: &Message, new_media: &InputMedia) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            media: new_media.to_owned(),
            inline_message_id: None,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_reply_markup`]
///
/// [`edit_message_reply_markup`]:
/// ../../api/trait.API.html#method.edit_message_reply_markup
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageReplyMarkup {
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message to edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the inline
    /// message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`stop_poll`]
///
/// [`stop_poll`]:
/// ../../api/trait.API.html#method.stop_poll
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StopPoll {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: message.chat.get_id().into(),
            message_id: message.message_id,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`delete_message`]
///
/// [`delete_message`]:
/// ../../api/trait.API.html#method.delete_message
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteMessage {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Identifier of the message to delete
    pub message_id: i64,
}

impl DeleteMessage {
    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: message.chat.get_id().into(),
            message_id: message.message_id,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_live_location`]
///
/// [`edit_message_live_location`]:
/// ../../api/trait.API.html#method.edit_message_live_location
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageLiveLocation {
    /// Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and
    /// 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Maximum distance for proximity alerts about approaching another chat
    /// member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}

/// struct for holding data needed to call
/// [`stop_message_live_location`]
///
/// [`stop_message_live_location`]:
/// ../../api/trait.API.html#method.edit_message_live_location
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StopMessageLiveLocation {
    /// Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Identifier of the message to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
