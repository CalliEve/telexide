//! This modules provides all the objects describing the payloads to be send to
//! the different telegram API endpoints

mod chat;
mod commands;
mod edit_messages;
mod games;
mod inline;
mod input_media;
mod other;
mod passport;
mod payments;
mod send_messages;
mod stickers;
mod updates;
mod webhooks;

pub use chat::*;
pub use commands::*;
pub use edit_messages::*;
pub use games::*;
pub use inline::*;
pub use input_media::*;
pub use other::*;
pub use passport::*;
pub use payments::*;
pub use send_messages::*;
pub use stickers::*;
pub use updates::{GetUpdates, UpdateType};
pub use webhooks::*;
