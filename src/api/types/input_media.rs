use super::InputFile;
use crate::model::{MessageEntity, ParseMode};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// This object represents the content of a media message to be sent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    #[serde(rename = "document")]
    Document(InputMediaDocument),
}

/// Represents a photo to be sent.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet
    pub media: InputFile,
    /// Caption of the photo to be sent, 0-1024 characters after entities
    /// parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass True if the photo needs to be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet
    pub media: InputFile,
    /// Caption of the video to be sent, 0-1024 characters after entities
    /// parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
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
    /// If the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Pass True if the video needs to be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound)
/// to be sent.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputMediaAnimation {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet
    pub media: InputFile,
    /// Caption of the animation to be sent, 0-1024 characters after entities
    /// parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Pass True if the animation needs to be covered with a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// Represents an audio file to be treated as music to be sent.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputMediaAudio {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet
    pub media: InputFile,
    /// Caption of the audio file to be sent, 0-1024 characters after entities
    /// parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Represents a general file to be sent.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputMediaDocument {
    /// File to send. Pass a file_id to send a file that exists on the Telegram
    /// servers (recommended), pass an HTTP URL for Telegram to get a file
    /// from the Internet
    pub media: InputFile,
    /// Caption of the document to be sent, 0-1024 characters after entities
    /// parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the caption, which can be
    /// specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded
    /// using multipart/form-data. Always true, if the document is sent as
    /// part of an album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

impl InputMedia {
    pub fn get_media(&self) -> &InputFile {
        match self {
            InputMedia::Photo(m) => &m.media,
            InputMedia::Video(m) => &m.media,
            InputMedia::Audio(m) => &m.media,
            InputMedia::Animation(m) => &m.media,
            InputMedia::Document(m) => &m.media,
        }
    }
}
