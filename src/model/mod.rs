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
pub mod raw;
mod stickers;
mod telegram_passport;
mod update;
mod user;

pub use chat::*;
pub use games::*;
pub use inline::*;
pub use markup::*;
pub use message::*;
pub use message_contents::*;
pub use message_entity::*;
pub use other::*;
pub use payments::*;
pub use stickers::*;
pub use telegram_passport::*;
pub use update::*;
pub use user::*;
