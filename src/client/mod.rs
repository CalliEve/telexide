//! The [Client] contains information about your registered event handlers.
//! Dispatching events to configured handlers is also handled directly via the
//! client. In addition, the [`api`] and [`framework`] modules are also
//! automatically handled by the Client module for you.
//!
//! A [`Context`] is provided for every handler.
//!
//! See [here][examples] for some examples on how to use the [`Client`].
//!
//! [examples]: struct.Client.hmtl#Examples
//! [`api`]: ../api/index.hmtl
//! [`framework`]: ../framework/index.html
//! [`Context`]: struct.Context.html

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
    EventHandler,
    InlineQueryHandler,
    InlineResultHandler,
    MessageHandler,
    RawEventHandler,
};
pub use stream::UpdatesStream;

type APIConnector = dyn API + Send + Sync;
pub(crate) type FutureOutcome = Pin<Box<dyn Future<Output = ()> + Send>>;
