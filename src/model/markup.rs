use super::{CallbackGame, LoginUrl};
use serde::{Deserialize, Serialize};

/// This object represents an [inline keyboard] that appears right next to the
/// message it belongs to.
///
/// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineKeyboardMarkup {
    /// Vec of button rows, each represented by a Vec of
    /// [`InlineKeyboardButton`] objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// This object represents one button of an inline keyboard.
/// You **must** use exactly one of the optional fields.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
/// [keyboards]: https://core.telegram.org/bots#keyboards
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReplyKeyboardMarkup {
    /// Vec of button rows, each represented by a Vec of [`KeyboardButton`]
    /// objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons).
    /// Defaults to false, in which case the custom keyboard is always of the
    /// same height as the app's standard keyboard.
    #[serde(default)]
    pub resize_keyboard: bool,
    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user can press a
    /// special button in the input field to see the custom keyboard again.
    /// Defaults to false.
    #[serde(default)]
    pub one_time_keyboard: bool,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters.
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
    #[serde(default)]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will remove
/// the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent by a
/// bot. An exception is made for one-time keyboards that are hidden immediately
/// after the user presses a button (see [`ReplyKeyboardMarkup`]).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(default)]
    pub selective: bool,
}

/// Upon receiving a message with this object, Telegram clients will display a
/// reply interface to the user (act as if the user has selected the bot‘s
/// message and tapped ’Reply'). This can be extremely useful if you want to
/// create user-friendly step-by-step interfaces without having to sacrifice
/// [privacy mode].
///
/// [privacy mode]: https://core.telegram.org/bots#privacy-mode
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters.
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to force reply from specific
    /// users only. Targets: 1) users that are @mentioned in the text of the
    /// [`Message`] object; 2) if the bot's message is a reply (has
    /// reply_to_message_id), sender of the original message.
    ///
    /// [`Message`]: ../model/struct.Message.html
    #[serde(default)]
    pub selective: bool,
}

/// This object represents one button of the reply keyboard.
/// For simple text buttons String can be used instead of this object to specify
/// text of the button.
///
/// **Note:** Optional fields `request_contact`, `request_location`, and
/// `request_poll` are mutually exclusive.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed
    pub text: String,
    /// If true, the user's phone number will be sent as a contact when the
    /// button is pressed. Available in private chats only
    pub request_contact: bool,
    /// If true, the user's current location will be sent when the button is
    /// pressed. Available in private chats only
    pub request_location: bool,
    /// If specified, the user will be asked to create a poll and send it to the
    /// bot when the button is pressed. Available in private chats only
    pub request_poll: Option<KeyboardButtonPollType>,
}

/// This object represents type of a poll, which is allowed to be created and
/// sent when the corresponding button is pressed.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardButtonPollType {
    /// If quiz is passed, the user will be allowed to create only polls in the
    /// quiz mode. If regular is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    pub poll_type: super::PollType,
}
