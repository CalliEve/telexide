use super::{InputFile, InputMedia};
use crate::{
    model::{
        utils::unix_date_formatting,
        ChatAction,
        MessageEntity,
        ParseMode,
        PhotoSize,
        PollType,
        ReplyMarkup,
    },
    prelude::Message,
    utils::result::Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// struct for holding data needed to call
/// [`send_message`]
///
/// [`send_message`]:
/// ../../api/trait.API.html#method.send_message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Text of the message to be sen, 1-4096 characters after entities parsing
    pub text: String,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in message text, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enitites: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: bool,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    pub fn new(chat_id: i64, text: &str) -> Self {
        Self {
            chat_id,
            text: text.to_owned(),
            parse_mode: None,
            enitites: None,
            disable_notification: false,
            disable_web_page_preview: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn set_parse_mode(&mut self, mode: &ParseMode) -> &mut Self {
        self.parse_mode = Some(mode.to_owned());
        self
    }

    pub fn reply_to_message(&mut self, message: &Message) -> &mut Self {
        self.reply_to_message_id = Some(message.message_id);
        self
    }

    pub fn set_reply_to_message_id(&mut self, id: i64) -> &mut Self {
        self.reply_to_message_id = Some(id);
        self
    }

    pub fn set_reply_markup(&mut self, markup: &ReplyMarkup) -> &mut Self {
        self.reply_markup = Some(markup.to_owned());
        self
    }

    pub fn toggle_disable_notification(&mut self) -> &mut Self {
        self.disable_notification = !self.disable_notification;
        self
    }

    pub fn toggle_disable_web_page_preview(&mut self) -> &mut Self {
        self.disable_web_page_preview = !self.disable_web_page_preview;
        self
    }
}

/// struct for holding data needed to call
/// [`forward_message`]
///
/// [`forward_message`]:
/// ../../api/trait.API.html#method.forward_message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier for the chat where the original message was sent.
    pub from_chat_id: i64,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
}

impl ForwardMessage {
    pub fn new(chat_id: i64, from_chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: false,
        }
    }

    pub fn toggle_disable_notification(&mut self) -> &mut Self {
        self.disable_notification = !self.disable_notification;
        self
    }

    pub fn from_message(chat_id: i64, message: &Message) -> Self {
        Self {
            chat_id,
            from_chat_id: message.chat.get_id(),
            message_id: message.message_id,
            disable_notification: false,
        }
    }
}

/// struct for holding data needed to call [`copy_message`]
///
/// [`copy_message`]: ../../api/trait.API.html#method.copy_message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CopyMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier for the chat where the original message was sent.
    pub from_chat_id: i64,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
    /// New caption for media, 0-1024 characters after entities parsing. If not
    /// specified, the original caption is kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Mode for parsing entities in the new caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl CopyMessage {
    pub fn new(chat_id: i64, from_chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            from_chat_id,
            message_id,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_message(chat_id: i64, from: &Message) -> Self {
        Self {
            chat_id,
            from_chat_id: from.chat.get_id(),
            message_id: from.message_id,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }
}

/// struct for holding data needed to call
/// [`send_photo`]
///
/// [`send_photo`]:
/// ../../api/trait.API.html#method.send_photo
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendPhoto {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Photo to send. Pass a file_id as String to send a photo that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a photo from the Internet
    pub photo: InputFile,
    /// Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPhoto {
    pub fn new(chat_id: i64, photo: String) -> Self {
        Self {
            chat_id,
            photo: InputFile::String(photo),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_photo_size(chat_id: i64, photo: &PhotoSize) -> Self {
        Self {
            chat_id,
            photo: InputFile::String(photo.file_id.clone()),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            photo: InputFile::from_path(path)?,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_audio`]
///
/// [`send_audio`]:
/// ../../api/trait.API.html#method.send_audio
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendAudio {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Audio to send. Pass a file_id as String to send an audio file that
    /// exists on the Telegram servers (recommended), pass an HTTP URL as a
    /// String for Telegram to get an audio file from the Internet
    pub audio: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Audio caption (may also be used when resending audio files by file_id),
    /// 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// The performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAudio {
    pub fn new(chat_id: i64, audio: String) -> Self {
        Self {
            chat_id,
            audio: InputFile::String(audio),
            thumb: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            performer: None,
            duration: None,
            title: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            audio: InputFile::from_path(path)?,
            thumb: None,
            caption: None,
            caption_entities: None,
            performer: None,
            duration: None,
            title: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_document`]
///
/// [`send_document`]:
/// ../../api/trait.API.html#method.send_document
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendDocument {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Document to send. Pass a file_id as String to send a photo that exists
    /// on the Telegram servers (recommended), pass an HTTP URL as a String
    /// for Telegram to get a document from the Internet
    pub document: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Document caption (may also be used when resending documents by file_id),
    /// 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables automatic server-side content type detection for files uploaded
    /// using multipart/form-data
    pub disable_content_type_detection: bool,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendDocument {
    pub fn new(chat_id: i64, document: String) -> Self {
        Self {
            chat_id,
            document: InputFile::String(document),
            thumb: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            disable_content_type_detection: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            document: InputFile::from_path(path)?,
            thumb: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            disable_content_type_detection: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_video`]
///
/// [`send_video`]:
/// ../../api/trait.API.html#method.send_video
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendVideo {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Video to send. Pass a file_id as String to send an video file that
    /// exists on the Telegram servers (recommended), pass an HTTP URL as a
    /// String for Telegram to get an video file from the Internet
    pub video: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Video caption (may also be used when resending video files by file_id),
    /// 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the uploaded video is suitable for streaming
    pub supports_streaming: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVideo {
    pub fn new(chat_id: i64, video: String) -> Self {
        Self {
            chat_id,
            video: InputFile::String(video),
            thumb: None,
            caption: None,
            caption_entities: None,
            duration: None,
            width: None,
            height: None,
            performer: None,
            title: None,
            supports_streaming: false,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            video: InputFile::from_path(path)?,
            thumb: None,
            caption: None,
            caption_entities: None,
            duration: None,
            width: None,
            height: None,
            performer: None,
            title: None,
            supports_streaming: false,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_animation`]
///
/// [`send_animation`]:
/// ../../api/trait.API.html#method.send_animation
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendAnimation {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Animation to send. Pass a file_id as String to send an animation file
    /// that exists on the Telegram servers (recommended), pass an HTTP URL
    /// as a String for Telegram to get an animation file from the Internet
    pub animation: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Animation caption (may also be used when resending animation files by
    /// file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendAnimation {
    pub fn new(chat_id: i64, animation: String) -> Self {
        Self {
            chat_id,
            animation: InputFile::String(animation),
            thumb: None,
            caption: None,
            caption_entities: None,
            duration: None,
            width: None,
            height: None,
            performer: None,
            title: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            animation: InputFile::from_path(path)?,
            thumb: None,
            caption: None,
            caption_entities: None,
            duration: None,
            width: None,
            height: None,
            performer: None,
            title: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_voice`]
///
/// [`send_voice`]:
/// ../../api/trait.API.html#method.send_voice
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendVoice {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Voice to send. Pass a file_id as String to send an voice file that
    /// exists on the Telegram servers (recommended), pass an HTTP URL as a
    /// String for Telegram to get an voice file from the Internet
    pub voice: InputFile,
    /// Voice caption (may also be used when resending video files by file_id),
    /// 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// List of special entities that appear in the new caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in your bot's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVoice {
    pub fn new(chat_id: i64, voice: String) -> Self {
        Self {
            chat_id,
            voice: InputFile::String(voice),
            caption: None,
            caption_entities: None,
            duration: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            voice: InputFile::from_path(path)?,
            duration: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for holding data needed to call
/// [`send_video_note`]
///
/// [`send_video_note`]:
/// ../../api/trait.API.html#method.send_video_note
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// VideoNote to send. Pass a file_id as String to send an video_note file
    /// that exists on the Telegram servers (recommended), pass an HTTP URL
    /// as a String for Telegram to get an video_note file from the Internet
    pub video_note: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVideoNote {
    pub fn new(chat_id: i64, note: String) -> Self {
        Self {
            chat_id,
            video_note: InputFile::String(note),
            thumb: None,
            duration: None,
            length: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    pub fn from_file<P: AsRef<Path>>(chat_id: i64, path: P) -> Result<Self> {
        Ok(Self {
            chat_id,
            video_note: InputFile::from_path(path)?,
            thumb: None,
            duration: None,
            length: None,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        })
    }
}

/// struct for sending photos, videos, documents or audios as an album
/// [`send_media_group`]
///
/// [`send_media_group`]:
/// ../../api/trait.API.html#method.send_media_group
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Photos, videos, documents or audios as an album to be send, amount must
    /// be 2-10
    pub media: Vec<InputMedia>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
}

impl SendMediaGroup {
    pub fn new(chat_id: i64, media: Vec<InputMedia>) -> Self {
        Self {
            chat_id,
            media,
            disable_notification: false,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
        }
    }
}

/// struct for holding data needed to call
/// [`send_location`]
///
/// [`send_location`]:
/// ../../api/trait.API.html#method.send_location
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendLocation {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// Period in seconds for which the location will be updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about
    /// approaching another chat member, in meters. Must be between 1 and
    /// 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`send_venue`]
///
/// [`send_venue`]:
/// ../../api/trait.API.html#method.send_venue
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendVenue {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known.
    /// (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`send_contact`]
///
/// [`send_contact`]:
/// ../../api/trait.API.html#method.send_contact
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendContact {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`send_poll`]
///
/// [`send_poll`]:
/// ../../api/trait.API.html#method.send_poll
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendPoll {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Poll question, 1-255 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-300 characters
    /// each
    pub options: Vec<String>,
    /// True, if the poll needs to be anonymous, defaults to True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub poll_type: Option<PollType>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz
    /// mode
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option, required for polls in
    /// quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line
    /// feeds after entities parsing
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the poll explanation, which can
    /// be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_enitites: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600.
    /// Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically
    /// closed. Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period.
    #[serde(with = "unix_date_formatting::optional")]
    pub close_date: Option<DateTime<Utc>>,
    /// Pass True, if the poll needs to be immediately closed.
    pub is_closed: bool,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`send_dice`]
///
/// [`send_dice`]:
/// ../../api/trait.API.html#method.send_dice
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendDice {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Emoji on which the dice throw animation is based.
    /// Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”.
    /// Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀”
    /// and “⚽”, and values 1-64 for “🎰”.
    /// Defauts to “🎲”
    pub emoji: Option<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified
    /// replied-to message is not found
    pub allow_sending_without_reply: bool,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`send_chat_action`]
///
/// [`send_chat_action`]:
/// ../../api/trait.API.html#method.send_chat_action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendChatAction {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Type of action to broadcast.
    pub action: ChatAction,
}
