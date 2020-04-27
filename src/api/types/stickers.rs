use super::InputFile;
use crate::model::{MaskPosition, ReplyMarkup};
use serde::{Deserialize, Serialize};

/// struct for holding data needed to call
/// [`send_sticker`]
///
/// [`send_sticker`]:
/// ../../api/trait.API.html#method.send_sticker
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendSticker {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Sticker to send. Pass a file_id as String to send a file that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a .WEBP file from the Internet, or upload a new one
    pub sticker: InputFile,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`get_sticker_set`]
///
/// [`get_sticker_set`]:
/// ../../api/trait.API.html#method.get_sticker_set
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

/// struct for holding data needed to call
/// [`upload_sticker_file`]
///
/// [`upload_sticker_file`]:
/// ../../api/trait.API.html#method.upload_sticker_file
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// Png image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px.
    pub png_sticker: InputFile,
}

/// struct for holding data needed to call
/// [`create_new_sticker_set`]
///
/// [`create_new_sticker_set`]:
/// ../../api/trait.API.html#method.create_new_sticker_set
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g.,
    /// animals). Can contain only english letters, digits and underscores.
    /// Must begin with a letter, can't contain consecutive underscores and
    /// must end in “_by_<bot username>”. <bot_username> is case
    /// insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. Pass a file_id as a String to send a file that
    /// already exists on the Telegram servers, ass an HTTP URL as a String
    /// for Telegram to get a file from the Internet, or upload a new one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// Pass True, if a set of mask stickers should be created
    pub contains_masks: bool,
    /// position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

/// struct for holding data needed to call
/// [`add_sticker_to_set`]
///
/// [`add_sticker_to_set`]:
/// ../../api/trait.API.html#method.add_sticker_to_set
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Name of the sticker set
    pub name: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size,
    /// dimensions must not exceed 512px, and either width or height must be
    /// exactly 512px. Pass a file_id as a String to send a file that
    /// already exists on the Telegram servers, ass an HTTP URL as a String
    /// for Telegram to get a file from the Internet, or upload a new one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data.
    /// See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

/// struct for holding data needed to call
/// [`set_sticker_position_in_set`]
///
/// [`set_sticker_position_in_set`]:
/// ../../api/trait.API.html#method.set_sticker_position_in_set
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set
    pub position: i64,
}

/// struct for holding data needed to call
/// [`delete_sticker_from_set`]
///
/// [`delete_sticker_from_set`]:
/// ../../api/trait.API.html#method.delete_sticker_from_set
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

/// struct for holding data needed to call
/// [`set_sticker_set_thumb`]
///
/// [`set_sticker_set_thumb`]:
/// ../../api/trait.API.html#method.set_sticker_set_thumb
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetStickerSetThumb {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and
    /// have width and height exactly 100px, or a TGS animation with the
    /// thumbnail up to 32 kilobytes in size; see https://core.telegram.org/animated_stickers#technical-requirements for animated sticker technical requirements.
    /// Pass a file_id as a String to send a file that already exists on the
    /// Telegram servers, pass an HTTP URL as a String for Telegram to get a
    /// file from the Internet, or upload a new one. Animated sticker set
    /// thumbnail can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
}
