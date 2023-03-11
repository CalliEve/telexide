use crate::{
    model::{ChatAdministratorRights, MenuButton},
    utils::{
        result::{Result, TelegramError},
        FormDataFile,
    },
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fs::File, path::Path};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`get_user_profile_photos`]
///
/// [`get_user_profile_photos`]:
/// ../../api/trait.API.html#method.get_user_profile_photos
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// struct for holding data needed to call
/// [`get_file`]
///
/// [`get_file`]:
/// ../../api/trait.API.html#method.get_file
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

/// struct for holding data needed to call
/// [`answer_callback_query`]
///
/// [`answer_callback_query`]:
/// ../../api/trait.API.html#method.answer_callback_query
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the
    /// user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification
    /// at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game
    /// and accepted the conditions via @Botfather, specify the URL that
    /// opens your game – note that this will only work if the query comes from
    /// a callback_game button.
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open
    /// your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback
    /// query may be cached client-side. Telegram apps will support caching
    /// starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

/// Is either true (the bool), or is object T
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum TrueOrObject<T> {
    True(bool),
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    Object(T),
}

/// This object represents either the `file_id`, http url or the contents of a
/// file to be uploaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFile {
    String(String),
    File(FormDataFile),
}

impl InputFile {
    pub fn new_file(file: &mut File, file_name: &str) -> Result<Self> {
        Ok(Self::File(FormDataFile::new_from_file(file, file_name)?))
    }

    pub fn new(string: &str) -> Self {
        Self::String(string.to_owned())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let file_name = path.as_ref().file_name().ok_or_else(|| {
            TelegramError::InvalidArgument("file doesn't have a valid file name".to_owned())
        })?;

        Self::new_file(
            &mut file,
            file_name.to_str().ok_or_else(|| {
                TelegramError::InvalidArgument("file doesn't have a valid file name".to_owned())
            })?,
        )
    }
}

impl From<String> for InputFile {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<&str> for InputFile {
    fn from(string: &str) -> Self {
        Self::String(string.to_owned())
    }
}

impl From<FormDataFile> for InputFile {
    fn from(file: FormDataFile) -> Self {
        Self::File(file)
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(ref c) => serializer.serialize_str(c),
            Self::File(ref c) => serializer.serialize_str(&format!(
                "attach://{}",
                &c.file_name
                    .as_ref()
                    .ok_or_else(|| serde::ser::Error::custom(
                        "file name doesn't exist for the InputFile file"
                    ))?
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D>(deserializer: D) -> std::result::Result<InputFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::String(Deserialize::deserialize(deserializer)?))
    }
}

/// struct for holding data needed to call
/// [`set_chat_menu_button`]
///
/// [`set_chat_menu_button`]:
/// ../../api/trait.API.html#method.set_chat_menu_button
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default
    /// bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to
    /// [`MenuButton::Default`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

/// struct for holding data needed to call
/// [`get_chat_menu_button`]
///
/// [`get_chat_menu_button`]:
/// ../../api/trait.API.html#method.get_chat_menu_button
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default
    /// bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}

/// struct for holding data needed to call
/// [`set_my_default_administrator_rights`]
///
/// [`set_my_default_administrator_rights`]:
/// ../../api/trait.API.html#method.set_my_default_administrator_rights
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyDefaultAdministratorRights {
    /// A JSON-serialized object describing new default administrator rights. If
    /// not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    /// Pass True to change the default administrator rights of the bot in
    /// channels. Otherwise, the default administrator rights of the bot for
    /// groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<bool>,
}

/// struct for holding data needed to call
/// [`set_my_default_administrator_rights`]
///
/// [`set_my_default_administrator_rights`]:
/// ../../api/trait.API.html#method.set_my_default_administrator_rights
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyDefaultAdministratorRights {
    /// Pass True to get the default administrator rights of the bot in
    /// channels. Otherwise, the default administrator rights of the bot for
    /// groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<bool>,
}

/// struct for holding data needed to call
/// [`set_my_description`]
///
/// [`set_my_description`]:
/// ../../api/trait.API.html#method.set_my_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyDescription {
    /// New bot description; 0-512 characters. Pass an empty string to remove
    /// the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be
    /// applied to all users for whose language there is no dedicated
    /// description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`get_my_description`]
///
/// [`get_my_description`]:
/// ../../api/trait.API.html#method.get_my_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyDescription {
    /// A two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`set_my_short_description`]
///
/// [`set_my_short_description`]:
/// ../../api/trait.API.html#method.set_my_short_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyShortDescription {
    /// New bot description; 0-120 characters. Pass an empty string to remove
    /// the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be
    /// applied to all users for whose language there is no dedicated
    /// description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`get_my_short_description`]
///
/// [`get_my_short_description`]:
/// ../../api/trait.API.html#method.get_my_short_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyShortDescription {
    /// A two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
