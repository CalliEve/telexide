use super::InputMedia;
use crate::model::{InlineKeyboardMarkup, Message, ParseMode};
use serde::{Deserialize, Serialize};

/// struct for holding data needed to call
/// [`edit_message_text`]
///
/// [`edit_message_text`]:
/// ../../api/trait.API.html#method.edit_message_text
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageText {
    /// Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: bool,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    fn new(chat_id: i64, message_id: i64, new_text: &str) -> Self {
        Self {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            text: new_text.to_owned(),
            inline_message_id: None,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_markup: None,
        }
    }

    fn from_message(message: &Message, new_text: &str) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            text: new_text.to_owned(),
            inline_message_id: None,
            parse_mode: None,
            disable_web_page_preview: false,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_caption`]
///
/// [`edit_message_caption`]:
/// ../../api/trait.API.html#method.edit_message_caption
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageCaption {
    /// Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    fn new(chat_id: i64, message_id: i64, new_text: Option<&str>) -> Self {
        Self {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            caption: new_text.map(ToString::to_string),
            inline_message_id: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    fn from_message(message: &Message, new_text: Option<&str>) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            caption: new_text.map(ToString::to_string),
            inline_message_id: None,
            parse_mode: None,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_media`]
///
/// [`edit_message_media`]:
/// ../../api/trait.API.html#method.edit_message_media
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageMedia {
    /// Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// new media content of the message
    pub media: InputMedia,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageMedia {
    fn new(chat_id: i64, message_id: i64, new_media: &InputMedia) -> Self {
        Self {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            media: new_media.to_owned(),
            inline_message_id: None,
            reply_markup: None,
        }
    }

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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditMessageReplyMarkup {
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
}

impl EditMessageReplyMarkup {
    fn new(chat_id: i64, message_id: i64, new_markup: Option<&InlineKeyboardMarkup>) -> Self {
        Self {
            chat_id: Some(chat_id),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: new_markup.cloned(),
        }
    }

    fn from_message(message: &Message, new_markup: Option<&InlineKeyboardMarkup>) -> Self {
        Self {
            chat_id: Some(message.chat.get_id()),
            message_id: Some(message.message_id),
            inline_message_id: None,
            reply_markup: new_markup.cloned(),
        }
    }
}

/// struct for holding data needed to call
/// [`stop_poll`]
///
/// [`stop_poll`]:
/// ../../api/trait.API.html#method.stop_poll
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StopPoll {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Identifier of the message to edit
    pub message_id: i64,
    /// Inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    fn new(chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
            reply_markup: None,
        }
    }

    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: message.chat.get_id(),
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Identifier of the message to delete
    pub message_id: i64,
}

impl DeleteMessage {
    fn new(chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
        }
    }

    fn from_message(message: &Message) -> Self {
        Self {
            chat_id: message.chat.get_id(),
            message_id: message.message_id,
        }
    }
}

/// struct for holding data needed to call
/// [`edit_message_live_location`]
///
/// [`edit_message_live_location`]:
/// ../../api/trait.API.html#method.edit_message_live_location
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
}

/// struct for holding data needed to call
/// [`stop_message_live_location`]
///
/// [`stop_message_live_location`]:
/// ../../api/trait.API.html#method.edit_message_live_location
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
