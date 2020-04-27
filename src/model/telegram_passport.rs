use super::utils::unix_date_formatting;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contains information about Telegram Passport data shared with the bot by the
/// user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportData {
    /// Vec with information about documents and other Telegram Passport
    /// elements that was shared with the bot
    pub data: Vec<EncryptedCredentials>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport.
/// Currently all Telegram Passport files are in JPEG format when decrypted and
/// don't exceed 10MB.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// File size
    pub file_size: i64,
    /// Unix time when the file was uploaded
    #[serde(with = "unix_date_formatting")]
    pub file_date: DateTime<Utc>,
}

/// Contains information about documents or other Telegram Passport elements
/// shared with the bot by the user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EncryptedPassportElement {
    /// Element type.
    pub element_type: TelegramPassportElement,
    /// Base64-encoded encrypted Telegram Passport element data provided by the
    /// user, available for “personal_details”, “passport”,
    /// “driver_license”, “identity_card”, “internal_passport” and “address”
    /// types. Can be decrypted and verified using the accompanying
    /// [EncryptedCredentials].
    pub data: Option<String>,
    /// User's verified phone number, available only for “phone_number” type
    pub phone_number: Option<String>,
    /// Array of encrypted files with documents provided by the user, available
    /// for “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types.
    /// Files can be decrypted and verified using the accompanying
    /// [EncryptedCredentials].
    pub files: Vec<PassportFile>,
    /// Encrypted file with the front side of the document, provided by the
    /// user. Available for “passport”, “driver_license”, “identity_card”
    /// and “internal_passport”. The file can be decrypted and verified
    /// using the accompanying [EncryptedCredentials].
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document, provided by the
    /// user. Available for “driver_license” and “identity_card”.
    /// The file can be decrypted and verified using the accompanying
    /// [EncryptedCredentials].
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided
    /// by the user; available for “passport”, “driver_license”,
    /// “identity_card” and “internal_passport”. The file can be decrypted
    /// and verified using the accompanying [EncryptedCredentials].
    pub selfie: PassportFile,
    /// Array of encrypted files with translated versions of documents provided
    /// by the user. Available if requested for “passport”,
    /// “driver_license”, “identity_card”, “internal_passport”,
    /// "utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types.
    /// Files can be decrypted and verified using the accompanying
    /// [EncryptedCredentials].
    pub translation: Vec<PassportFile>,
    /// Base64-encoded element hash for using in
    /// [PassportElementErrorUnspecified]
    ///
    /// [PassportElementErrorUnspecified]:
    /// ../api/types/struct.PassportElementErrorUnspecified.html
    pub hash: String,
}

/// Contains data required for decrypting and authenticating
/// [`EncryptedPassportElement`]. See the [Telegram Passport Documentation](https://core.telegram.org/passport#receiving-information)
/// for a complete description of the data decryption and authentication
/// processes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's
    /// payload, data hashes and secrets required for
    /// [EncryptedPassportElement] decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required
    /// for data decryption
    pub secret: String,
}

/// The type of a telegram passport element
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TelegramPassportElement {
    #[serde(rename = "personal_details")]
    PersonalDetails,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
    #[serde(rename = "address")]
    Address,
    #[serde(rename = "utility_bill")]
    UtilityBill,
    #[serde(rename = "bank_statement")]
    BankStatement,
    #[serde(rename = "rental_agreement")]
    RentalAgreement,
    #[serde(rename = "passport_registration")]
    PassportRegistration,
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "email")]
    Email,
}
