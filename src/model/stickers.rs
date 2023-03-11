use crate::api::types::InputFile;

use super::{File, PhotoSize};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// This object represents a sticker.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Type of the sticker. The type of the sticker is independent from its
    /// format, which is determined by the fields is_animated and is_video.
    #[serde(rename = "type")]
    pub kind: StickerType,
    /// Sticker width
    pub width: usize,
    /// Sticker height
    pub height: usize,
    /// True, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    #[serde(default)]
    pub is_animated: bool,
    /// True, if the sticker is a [video sticker](https://telegram.org/blog/video-stickers-better-reactions)
    #[serde(default)]
    pub is_video: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For premium regular stickers, premium animation for the sticker
    pub premium_animation: Option<File>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    pub custom_emoji_id: Option<String>,
    /// True, if the sticker must be repainted to a text color in messages, the
    /// color of the Telegram Premium badge in emoji status, white color on chat
    /// photos, or another appropriate color in other places
    #[serde(default)]
    pub needs_repainting: bool,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents a sticker set.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of “regular”, “mask”,
    /// “custom_emoji”
    pub sticker_type: StickerType,
    /// True, if the sticker set contains [animated stickers](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP or .TGS format
    pub thumbnail: Option<PhotoSize>,
}

/// This object describes a sticker to be added to a sticker set.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[build_struct]
pub struct InputSticker {
    /// The added sticker. Pass a file_id as a String to send a file that
    /// already exists on the Telegram servers, pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. Animated and video stickers can't be uploaded via
    /// HTTP URL.
    pub sticker: InputFile,
    /// List of 1-20 emoji associated with the sticker.
    pub emoji_list: Vec<String>,
    /// position where the mask should be placed on faces. For “mask” stickers
    /// only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// List of 0-20 search keywords for the sticker with total length of up to
    /// 64 characters. For “regular” and “custom_emoji” stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// This object describes the position on faces where a mask should be placed by
/// default.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[build_struct]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed
    pub point: MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size,
    /// from left to right. For example, choosing -1.0 will place mask just
    /// to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size,
    /// from top to bottom. For example, 1.0 will place the mask just below
    /// the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}

/// The part of the face relative to which a mask should be placed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MaskPoint {
    #[serde(rename = "forehead")]
    Forehead,
    #[serde(rename = "eyes")]
    Eyes,
    #[serde(rename = "mouth")]
    Mouth,
    #[serde(rename = "chin")]
    Chin,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum StickerType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "mask")]
    Mask,
    #[serde(rename = "custom_emoji")]
    CustomEmoji,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum StickerFormat {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "animated")]
    Animated,
    #[serde(rename = "video")]
    Video,
}
