use super::{CallbackGame, LoginUrl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct InlineKeyboardMarkup {
    pub text: String,
    pub url: Option<String>,
    pub login_url: Option<LoginUrl>,
    pub callback_data: Option<String>,
    pub switch_inline_query: Option<String>,
    pub switch_inline_query_current_chat: Option<String>,
    pub callback_game: Option<CallbackGame>,
    #[serde(default)]
    pub pay: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<KeyboardButton>,
    #[serde(default)]
    pub resize_keyboard: bool,
    #[serde(default)]
    pub one_time_keyboard: bool,
    #[serde(default)]
    pub selective: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ReplyKeyboardRemove {
    // warning: this field always has to be true
    pub remove_keyboard: bool,
    #[serde(default)]
    pub selective: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ForceReply {
    // warning: this field always has to be true
    pub force_reply: bool,
    #[serde(default)]
    pub selective: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: bool,
    pub request_location: bool,
    pub request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct KeyboardButtonPollType {
    #[serde(rename = "type")]
    pub poll_type: super::PollType,
}
