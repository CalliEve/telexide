use crate::model::TelegramPassportElement;
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`set_passport_data_errors`]
///
/// [`set_passport_data_errors`]:
/// ../../api/trait.API.html#method.set_passport_data_errors
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A vec describing the errors
    pub errors: Vec<PassportElementError>,
}

/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "source")]
pub enum PassportElementError {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),
    #[serde(rename = "front_side")]
    FrontSide(PassportElementErrorFrontSide),
    #[serde(rename = "reverse_side")]
    ReverseSide(PassportElementErrorReverseSide),
    #[serde(rename = "selfie")]
    Selfie(PassportElementErrorSelfie),
    #[serde(rename = "file")]
    File(PassportElementErrorFile),
    #[serde(rename = "files")]
    Files(PassportElementErrorFiles),
    #[serde(rename = "translation_file")]
    TranslationFile(PassportElementErrorTranslationFile),
    #[serde(rename = "translation_files")]
    TranslationFiles(PassportElementErrorTranslationFiles),
    #[serde(rename = "unspecified")]
    Unspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the user.
/// The error is considered resolved when the field's value changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error,
    /// one of “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”, “address”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the front side of a document.
/// The error is considered resolved when the file with the front side of the
/// document changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue,
    /// one of “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the reverse side of a document.
/// The error is considered resolved when the file with reverse side of the
/// document changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue,
    /// one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the selfie with a document.
/// The error is considered resolved when the file with the selfie changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue,
    /// one of “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a document scan.
/// The error is considered resolved when the file with the document scan
/// changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue
    /// one of “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a list of scans.
/// The error is considered resolved when the list of files containing the scans
/// changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue,
    /// one of “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue with one of the files that constitute the translation of
/// a document. The error is considered resolved when the file changes.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the issue,
    /// one of “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the translated version of a document.
/// The error is considered resolved when a file with the document translation
/// change.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue,
    /// one of “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue in an unspecified place.
/// The error is considered resolved when new data is added.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub section_type: TelegramPassportElement,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}
