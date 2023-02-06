use super::{CallbackGame, ChatAdministratorRights, LoginUrl};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// This object represents an [inline keyboard] that appears right next to the
/// message it belongs to.
///
/// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineKeyboardMarkup {
    /// Vec of button rows, each represented by a Vec of
    /// [`InlineKeyboardButton`] objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents one button of an inline keyboard.
/// You **must** use exactly one of the optional fields.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// HTTP or tg:// url to be opened when button is pressed
    pub url: Option<String>,
    /// An HTTP URL used to automatically authorize the user.
    /// Can be used as a replacement for the [Telegram Login Widget][widget].
    ///
    /// [widget]: https://core.telegram.org/widgets/login
    pub login_url: Option<LoginUrl>,
    /// Data to be sent in a [callback query] to the bot when button is pressed,
    /// 1-64 bytes
    ///
    /// [callback query]: ../model/struct.CallbackQuery.html
    pub callback_data: Option<String>,
    /// Description of the Web App that will be launched when the user presses
    /// the button. The Web App will be able to send an arbitrary message on
    /// behalf of the user using the method [`answer_web_app_query`]. Available
    /// only in private chats between a user and the bot.
    ///
    /// [`answer_web_app_query`]: ../api/trait.API.html#method.answer_web_app_query
    pub web_app: Option<WebAppInfo>,
    /// If set, pressing the button will prompt the user to select one of their
    /// chats, open that chat and insert the bot‘s username and the
    /// specified inline query in the input field. Can be empty, in which
    /// case just the bot’s username will be inserted.
    ///
    /// **Note:** This offers an easy way for users to start using your bot in
    /// inline mode when they are currently in a private chat with it.
    /// Especially useful when combined with switch_pm… actions – in this case
    /// the user will be automatically returned to the chat they switched
    /// from, skipping the chat selection screen.
    ///
    /// [inline mode]: https://core.telegram.org/bots/inline
    /// [switch_pm]: ../api/trait.API.html#method.answer_callback_query
    pub switch_inline_query: Option<String>,
    /// If set, pressing the button will insert the bot‘s username and the
    /// specified inline query in the current chat's input field. Can be
    /// empty, in which case only the bot’s username will be inserted.
    ///
    /// This offers a quick way for the user to open your bot in inline mode in
    /// the same chat – good for selecting something from multiple options.
    pub switch_inline_query_current_chat: Option<String>,
    /// Description of the game that will be launched when the user presses the
    /// button.
    ///
    /// **NOTE:** This type of button must always be the first button in the
    /// first row.
    pub callback_game: Option<CallbackGame>,
    /// Specify True, to send a [Pay button].
    ///
    /// **NOTE:** This type of button must always be the first button in the
    /// first row.
    ///
    /// [Pay button]: https://core.telegram.org/bots/api#payments
    #[serde(default)]
    pub pay: bool,
}

/// This object represents a custom keyboard with reply options
/// (see [Introduction to bots][keyboards] for details and examples).
///
/// [keyboards]: https://core.telegram.org/bots/features#keyboards
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplyKeyboardMarkup {
    /// Vec of button rows, each represented by a Vec of [`KeyboardButton`]
    /// objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to always show the keyboard when the regular keyboard
    /// is hidden. Defaults to false, in which case the custom keyboard can be
    /// hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the
    /// same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user can press a
    /// special button in the input field to see the custom keyboard again.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// The placeholder to be shown in the input field when the keyboard is
    /// active; 1-64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users
    /// only. Targets: 1) users that are @mentioned in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply (has
    /// reply_to_message_id), sender of the original message.
    ///
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with akeyboard to select the new language.
    /// Other users in the group don’t see the keyboard.
    ///
    /// [`Message`]: ../model/struct.Message.html
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// Upon receiving a message with this object, Telegram clients will remove
/// the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent by a
/// bot. An exception is made for one-time keyboards that are hidden immediately
/// after the user presses a button (see [`ReplyKeyboardMarkup`]).
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to
    /// summon this keyboard; if you want to hide the keyboard from sight
    /// but keep it accessible, use one_time_keyboard in
    /// [`ReplyKeyboardMarkup`])
    ///
    /// **warning:** this field always has to be true
    pub remove_keyboard: bool,
    /// Use this parameter if you want to remove the keyboard for specific users
    /// only. Targets: 1) users that are @mentioned in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply (has
    /// reply_to_message_id), sender of the original message.
    ///
    /// Example: A user votes in a poll, bot returns confirmation message in
    /// reply to the vote and removes the keyboard for that user, while
    /// still showing the keyboard with poll options to users who haven't voted
    /// yet.
    ///
    /// [`Message`]: ../model/struct.Message.html
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// Upon receiving a message with this object, Telegram clients will display a
/// reply interface to the user (act as if the user has selected the bot‘s
/// message and tapped ’Reply'). This can be extremely useful if you want to
/// create user-friendly step-by-step interfaces without having to sacrifice
/// [privacy mode].
///
/// [privacy mode]: https://core.telegram.org/bots#privacy-mode
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForceReply {
    /// Upon receiving a message with this object, Telegram clients will display
    /// a reply interface to the user (act as if the user has selected the bot‘s
    /// message and tapped ’Reply'). This can be extremely useful if you
    /// want to create user-friendly step-by-step interfaces without having to
    /// sacrifice [privacy mode].
    ///
    /// **warning:** this field always has to be true
    ///
    /// [privacy mode]: https://core.telegram.org/bots#privacy-mode
    pub force_reply: bool,
    /// The placeholder to be shown in the input field when the keyboard is
    /// active; 1-64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to force reply from specific
    /// users only. Targets: 1) users that are @mentioned in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply (has
    /// reply_to_message_id), sender of the original message.
    ///
    /// [`Message`]: ../model/struct.Message.html
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// This object represents one button of the reply keyboard.
/// For simple text buttons String can be used instead of this object to specify
/// text of the button.
///
/// **Note:** Optional fields `request_contact`, `request_location`, and
/// `request_poll` are mutually exclusive.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed
    pub text: String,
    /// If specified, pressing the button will open a list of suitable users.
    /// Tapping on any user will send their identifier to the bot in a
    /// “user_shared” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_user: Option<KeyboardButtonRequestUser>,
    /// If specified, pressing the button will open a list of suitable chats.
    /// Tapping on a chat will send its identifier to the bot in a “chat_shared”
    /// service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// If true, the user's phone number will be sent as a contact when the
    /// button is pressed. Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// If true, the user's current location will be sent when the button is
    /// pressed. Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// If specified, the user will be asked to create a poll and send it to the
    /// bot when the button is pressed. Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// If specified, the described Web App will be launched when the button is
    /// pressed. The Web App will be able to send a “web_app_data” service
    /// message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

/// This object represents type of a poll, which is allowed to be created and
/// sent when the corresponding button is pressed.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyboardButtonPollType {
    /// If quiz is passed, the user will be allowed to create only polls in the
    /// quiz mode. If regular is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    pub poll_type: super::PollType,
}

/// Describes a [Web App].
///
/// [Web App]: https://core.telegram.org/bots/webapps
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified
    /// in [Initializing Web Apps]
    ///
    /// [Initializing Web Apps]: https://core.telegram.org/bots/webapps#initializing-web-apps
    pub url: String,
}

/// This object defines the criteria used to request a suitable user. The
/// identifier of the selected user will be shared with the bot when the
/// corresponding button is pressed.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyboardButtonRequestUser {
    /// Signed 32-bit identifier of the request, which will be received back in
    /// the [`UserShared`] object. Must be unique within the message
    ///
    /// [`UserShared`]: ../model/struct.UserShared.html
    pub request_id: i32,
    /// Pass True to request a bot, pass False to request a regular user. If not
    /// specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// Pass True to request a premium user, pass False to request a non-premium
    /// user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
}

/// This object defines the criteria used to request a suitable chat. The
/// identifier of the selected chat will be shared with the bot when the
/// corresponding button is pressed.
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in
    /// the [`ChatShared`] object. Must be unique within the message
    ///
    /// [`ChatShared`]: ../model/struct.ChatShared.html
    pub request_id: i32,
    /// Pass True to request a channel chat, pass False to request a group or a
    /// supergroup chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_channel: Option<bool>,
    /// Pass True to request a forum supergroup, pass False to request a
    /// non-forum chat. If not specified, no additional restrictions are
    /// applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// Pass True to request a supergroup or a channel with a username, pass
    /// False to request a chat without a username. If not specified, no
    /// additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// Pass True to request a chat owned by the user. Otherwise, no additional
    /// restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// An object listing the required administrator rights of the user in the
    /// chat. The rights must be a superset of bot_administrator_rights. If not
    /// specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// An object listing the required administrator rights of the bot in the
    /// chat. The rights must be a subset of user_administrator_rights. If not
    /// specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// Pass True to request a chat with the bot as a member. Otherwise, no
    /// additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
}
