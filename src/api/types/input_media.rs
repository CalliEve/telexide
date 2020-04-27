use super::InputFile;
use crate::model::ParseMode;
use serde::{Deserialize, Serialize};

/// This object represents the content of a media message to be sent
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub supports_streaming: bool,
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound)
/// to be sent.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    /// Duration of the animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
