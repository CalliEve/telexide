use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::fs::File;
use crate::utils::{FormDataFile, result::Result, result::TelegramError};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If true, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    pub show_alert: bool,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @Botfather,
    /// specify the URL that opens your game – note that this will only work if the query comes from a callback_game button.
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side.
    /// Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TrueOrObject<T>
{
    True(bool),
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    Object(T)
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputFile {
    String(String),
    File(FormDataFile)
}

impl InputFile {
    pub fn new_file(mut file: &mut File, file_name: &str) -> Result<Self> {
        Ok(Self::File(FormDataFile::new_from_file(&mut file, file_name)?))
    }

    pub fn new(string: &str) -> Self {
        Self::String(string.to_owned())
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let file_name = path.as_ref().file_name().ok_or_else(|| TelegramError::InvalidArgument("file doesn't have a valid file name".to_owned()))?;

        Self::new_file(&mut file, file_name.to_str().ok_or_else(|| TelegramError::InvalidArgument("file doesn't have a valid file name".to_owned()))?)
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
            Self::String(ref c) => {
                serializer.serialize_str(c)
            },
            Self::File(ref c) => {
                serializer.serialize_str(&format!("attach://{}", &c.file_name.as_ref().ok_or_else(|| serde::ser::Error::custom("file name doesn't exist for the InputFile file"))?))
            }
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
