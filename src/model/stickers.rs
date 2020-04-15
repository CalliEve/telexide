use super::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub is_animated: bool,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub set_name: Option<String>,
    pub mask_position: Option<MaskPosition>,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MaskPosition {
    pub point: MaskPoint,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

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
