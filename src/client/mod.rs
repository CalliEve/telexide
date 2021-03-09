//! The [`Client`] manages your registered event handlers and provides them with
//! up-to-date access to the api connection.
//!
//! The act of dispatching events to configured handlers is also handled
//! directly via the [`Client`]. In addition, the [`api`] and [`framework`]
//! modules are also automatically handled by the [`Client`] for you.
//!
//! A [`Context`] is provided for every handler, providing them with access to
//! the api connection and any shared data you have set.
//!
//! See [here][examples] for some examples on how to use the [`Client`].
//!
//! [examples]: struct.Client.html#Examples
//! [`api`]: ../api/index.html
//! [`framework`]: ../framework/index.html
//! [`Context`]: struct.Context.html
//! [`Client`]: struct.Client.html

mod builder;
mod client;
mod context;
mod event_handlers;
mod stream;
mod webhook_handling;

use crate::api::API;
use core::future::Future;
use std::pin::Pin;

pub use builder::ClientBuilder;
pub use client::Client;
pub use context::Context;
pub use event_handlers::{EventHandlerFunc, RawEventHandlerFunc};
pub use stream::UpdatesStream;
pub use webhook_handling::{Webhook, WebhookOptions};

type APIConnector = dyn API + Send;
pub(crate) type FutureOutcome = Pin<Box<dyn Future<Output = ()> + Send>>;
