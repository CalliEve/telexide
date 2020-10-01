//! Mappings of objects received from the API

pub(crate) mod utils;

mod chat;
mod games;
mod inline;
mod markup;
mod message;
mod message_contents;
mod message_entity;
mod other;
mod payments;
mod stickers;
mod telegram_passport;
mod update;
mod user;

pub use chat::{
    ChannelChat,
    Chat,
    ChatMember,
    ChatPermissions,
    ChatPhoto,
    ChatType,
    GroupChat,
    MemberMemberStatus,
    PrivateChat,
    SuperGroupChat,
};
pub use games::*;
pub use inline::*;
pub use markup::{
    ForceReply,
    InlineKeyboardMarkup,
    KeyboardButton,
    KeyboardButtonPollType,
    ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};
pub use message::{Message, MessageContent, ForwardData};
pub use message_contents::*;
pub use message_entity::*;
pub use other::{BotCommand, CallbackQuery, ChatAction, File, ParseMode, ReplyMarkup, WebhookInfo};
pub use payments::*;
pub use stickers::*;
pub use telegram_passport::*;
pub use update::{Update, UpdateContent};
pub use user::{User, UserProfilePhotos};

pub mod raw {
    pub use super::{chat::RawChat, message::RawMessage, update::RawUpdate};
}
