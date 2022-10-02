//! Note: It is necessary to enable [inline feedback] via [@Botfather] in order
//! to receive these objects in updates.
//!
//! [inline feedback]: https://core.telegram.org/bots/inline#collecting-feedback
//! [@Botfather]: https://t.me/botfather

use super::{ChatType, Location, User};
use serde::{Deserialize, Serialize};

/// This object represents an incoming inline query.
/// When the user sends an empty query, your bot could return some default or
/// trending results.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Sender location, only for bots that request user location
    pub location: Option<Location>,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Type of the chat, from which the inline query was sent. Can be either
    /// “sender” for a private chat with the inline query sender, “private”,
    /// “group”, “supergroup”, or “channel”. The chat type should be always
    /// known for requests sent from official clients and most third-party
    /// clients, unless the request was sent from a secret chat.
    pub chat_type: Option<ChatType>,
}

/// Represents a result of an inline query that was chosen by the user and sent
/// to their chat partner.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message.
    /// Available only if there is an inline keyboard attached to the message.
    /// Will be also received in callback queries and can be used to edit the
    /// message.
    pub query: String,
    /// The query that was used to obtain the result
    pub inline_message_id: Option<String>,
}

/// Describes an inline message sent by a [Web App] on behalf of a user.
///
/// [Web App]: https://core.telegram.org/bots/webapps
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    pub inline_message_id: Option<String>,
}
