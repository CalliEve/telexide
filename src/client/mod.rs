mod builder;
mod client;
mod context;
pub(crate) mod event_handlers;
mod stream;

use crate::api::API;
use core::future::Future;
use std::pin::Pin;

pub use builder::ClientBuilder;
pub use client::Client;
pub use context::Context;
pub use event_handlers::{
    MessageHandler, InlineQueryHandler, EventHandler, RawEventHandler, InlineResultHandler
};
pub use stream::UpdatesStream;

type APIConnector = dyn API + Send + Sync;
pub(crate) type FutureOutcome = Pin<Box<dyn Future<Output = ()> + Send>>;
