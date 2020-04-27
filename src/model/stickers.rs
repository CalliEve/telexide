use super::PhotoSize;
use serde::{Deserialize, Serialize};

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
    /// Sticker width
    pub width: usize,
    /// Sticker height
    pub height: usize,
    /// True, if the sticker is [animated](https://telegram.org/blog/animated-stickers)
    #[serde(default)]
    pub is_animated: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<String>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
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
    /// True, if the sticker set contains [animated stickers](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// True, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP or .TGS format
    pub thumb: Option<PhotoSize>,
}

/// This object describes the position on faces where a mask should be placed by
/// default.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
