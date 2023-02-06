use super::{utils::unix_date_formatting, User};
use crate::model::MessageEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents an audio file to be treated as music by the Telegram
/// clients.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: usize,
    /// Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
    /// Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}

/// This object represents a general file (as opposed to [photos][PhotoSize],
/// [voice messages][Voice] and [audio files][Audio]).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video
/// without sound).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: usize,
    /// Video height as defined by sender
    pub height: usize,
    /// Duration of the video in seconds as defined by sender
    pub duration: usize,
    /// Animation thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
    /// Original animation filename as defined by sender
    pub file_name: Option<String>,
}

/// This object represents one size of a photo or a file / sticker thumbnail
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Photo width
    pub width: usize,
    /// Photo height
    pub height: usize,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents a video file
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: usize,
    /// Video height as defined by sender
    pub height: usize,
    /// Duration of the video in seconds as defined by sender
    pub duration: usize,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents a voice note
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: usize,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents a [video message](https://telegram.org/blog/video-messages-and-telescope)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the
    /// file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over
    /// time and for different bots. Can't be used to download or reuse the
    /// file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by
    /// sender
    pub length: usize,
    /// Duration of the video in seconds as defined by sender
    pub duration: usize,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<usize>,
}

/// This object represents a phone contact.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a [vCard]
    ///
    /// [vCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,
}

/// This object represents a point on the map.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500.
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to the message sending date, during which the location can
    /// be updated, in seconds. For active live locations only.
    pub live_period: Option<i64>,
    /// The direction in which user is moving, in degrees; 1-360. For active
    /// live locations only.
    pub heading: Option<i64>,
    /// Maximum distance for proximity alerts about approaching another chat
    /// member, in meters. For sent live locations only.
    pub proximity_alert_radius: Option<i64>,
}

/// This object represents a venue.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    /// (For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [supported types].)
    ///
    /// [supported types]: https://developers.google.com/places/web-service/supported_types
    pub google_place_type: Option<String>,
}

/// This object contains information about a poll.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-255 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: usize,
    /// True, if the poll is closed
    #[serde(default)]
    pub is_closed: bool,
    /// True, if the poll is anonymous
    #[serde(default)]
    pub is_anonymous: bool,
    /// True, if the poll allows multiple answers
    #[serde(default)]
    pub allows_multiple_answers: bool,
    /// Poll type
    #[serde(rename = "type")]
    pub poll_type: PollType,
    /// 0-based identifier of the correct answer option.
    /// Available only for polls in the quiz mode, which are closed,
    /// or was sent (not forwarded) by the bot or to the private chat with the
    /// bot.
    pub correct_option_id: Option<usize>,
    /// Text that is shown when a user chooses an incorrect answer or taps on
    /// the lamp icon in a quiz-style poll, 0-200 characters
    pub explanation: Option<String>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in
    /// the explanation
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// Point in time when the poll will be automatically closed
    #[serde(default)]
    #[serde(with = "unix_date_formatting::optional")]
    pub close_date: Option<DateTime<Utc>>,
}

/// This object represents a dice with a random value from 1 to 6 for currently
/// supported base emoji.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀”
    /// and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: u8,
}

/// This object contains information about one answer option in a poll.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: usize,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user.
    /// May be empty if the user retracted their vote
    pub option_ids: Vec<usize>,
}

/// The type of the [`Poll`]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PollType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "quiz")]
    Quiz,
}

/// This object represents a parameter of the inline keyboard button used to
/// automatically authorize a user. Serves as a great replacement for the
/// [Telegram Login Widget] when the user is coming from Telegram. All the user
/// needs to do is tap/click a button and confirm that they want to log in
///
/// [Telegram Login Widget]: https://core.telegram.org/widgets/login
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LoginUrl {
    /// An HTTP URL to be opened with user authorization data added to the query
    /// string when the button is pressed. If the user refuses to provide
    /// authorization data, the original URL without information about the user
    /// will be opened. The data added is the same as described in
    /// [Receiving authorization data].
    ///
    /// **NOTE:** You must always check the hash of the received data to verify
    /// the authentication and the integrity of the data as described in
    /// [Checking authorization].
    ///
    /// [Receiving authorization data]: https://core.telegram.org/widgets/login#receiving-authorization-data
    /// [Checking authorization]: https://core.telegram.org/widgets/login#checking-authorization
    pub url: String,
    /// New text of the button in forwarded messages.
    pub forward_text: Option<String>,
    /// Username of a bot, which will be used for user authorization. See
    /// [Setting up a bot][setup] for more details. If not specified, the
    /// current bot's username will be assumed. The url's domain must be the
    /// same as the domain linked with the bot. See [Linking your domain to
    /// the bot][linking] for more details.
    ///
    /// [setup]: https://core.telegram.org/widgets/login#setting-up-a-bot
    /// [linking]: https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot
    pub bot_username: Option<String>,
    /// Pass True to request the permission for your bot to send messages to the
    /// user.
    #[serde(default)]
    pub request_write_access: bool,
}

/// This object represents the content of a service message, sent whenever a
/// user in the chat triggers a proximity alert set by another user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}

/// This object represents a service message about a voice chat scheduled in the
/// chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoChatScheduled {
    /// Point in time when the voice chat is supposed to be started by a chat
    /// administrator
    #[serde(with = "unix_date_formatting")]
    pub start_date: DateTime<Utc>,
}

/// This object represents a service message about a voice chat started in the
/// chat. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoChatStarted {}

/// This object represents a service message about a voice chat ended in the
/// chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoChatEnded {
    /// Voice chat duration; in seconds
    pub duration: i64,
}

/// This object represents a service message about new members invited to a
/// voice chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the voice chat.
    #[serde(default)]
    pub users: Option<Vec<User>>,
}

/// This object represents a service message about a change in auto-delete timer
/// settings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat
    pub message_auto_delete_time: i64,
}

/// Describes data sent from a [Web App] to the bot.
///
/// [Web App]: https://core.telegram.org/bots/webapps
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this
    /// field.
    pub data: String,
    /// Text of the web_app keyboard button from which the Web App was opened.
    /// Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}

/// This object represents a service message about a new forum topic created in
/// the chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about an edited forum topic.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicEdited {
    /// Name of the topic, if it was edited
    pub name: Option<String>,
    /// Unique identifier of the custom emoji shown as the topic icon, if it was
    /// edited; an empty string if the icon was removed
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic closed in the
/// chat. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicClosed {}

/// This object represents a service message about a forum topic reopened in the
/// chat. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicReopened {}

/// This object represents a service message about General forum topic hidden in
/// the chat. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GeneralForumTopicHidden {}

/// This object represents a service message about General forum topic unhidden
/// in the chat. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GeneralForumTopicUnhidden {}

/// This object represents a service message about a user allowing a bot added
/// to the attachment menu to write messages. Currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WriteAccessAllowed {}

/// This object contains information about the user whose identifier was shared
/// with the bot using a [`KeyboardButtonRequestUser`] button.
///
/// [`KeyboardButtonRequestUser`]: ../model/struct.KeyboardButtonRequestUser.html
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared user.
    /// The bot may not have access to the user and could be unable to use this
    /// identifier, unless the user is already known to the bot by some other
    /// means.
    pub user_id: i64,
}

/// This object contains information about the user whose identifier was shared
/// with the bot using a [`KeyboardButtonRequestChat`] button.
///
/// [`KeyboardButtonRequestChat`]: ../model/struct.KeyboardButtonRequestChat.html
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat.
    /// The bot may not have access to the chat and could be unable to use this
    /// identifier, unless the chat is already known to the bot by some other
    /// means.
    pub chat_id: i64,
}
