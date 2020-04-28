use super::PhotoSize;
use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
    /// True, if the bot can be invited to groups. Returned only in [`get_me`].
    ///
    /// [`get_me`]: ../api/struct.API.html#method.get_me
    pub can_join_groups: Option<bool>,
    /// True, if privacy mode is disabled for the bot. Returned only in
    /// [`get_me`].
    ///
    /// [`get_me`]: ../api/struct.API.html#method.get_me
    pub can_read_all_group_messages: Option<bool>,
    /// True, if the bot supports inline queries. Returned only in [`get_me`].
    ///
    /// [`get_me`]: ../api/struct.API.html#method.get_me
    pub supports_inline_queries: Option<bool>,
}

/// This object represent a user's profile pictures.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
