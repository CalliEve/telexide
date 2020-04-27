use super::{Context, FutureOutcome};
use crate::model::{raw::RawUpdate, ChosenInlineResult, InlineQuery, Message, Update};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) type MessageHandlerFunc = fn(Context, Message) -> FutureOutcome;
pub(crate) type InlineQueryHandlerFunc = fn(Context, InlineQuery) -> FutureOutcome;
pub(crate) type InlineResultHandlerFunc = fn(Context, ChosenInlineResult) -> FutureOutcome;

/// A function that handles a new update, it receives a [`Context`] and
/// [`Update`] and returns a pinned future. Wrap an async function with
/// [`#[prepare_listener]`] for easier development.
///
/// [`#[prepare_listener]`]: ../macros/macro.prepare_listener.html
#[allow(clippy::doc_markdown)]
pub(crate) type EventHandlerFunc = fn(Context, Update) -> FutureOutcome;

/// A function that handles a new raw update, it receives a [`Context`] and
/// [`RawUpdate`] and returns a pinned future. Wrap an async function with
/// [`#[prepare_listener]`] for easier development.
///
/// [`#[prepare_listener]`]: ../macros/macro.prepare_listener.html
#[allow(clippy::doc_markdown)]
pub(crate) type RawEventHandlerFunc = fn(Context, RawUpdate) -> FutureOutcome;

// is public for use with the command framework
#[doc(hidden)]
#[derive(Clone)]
pub struct EventHandler {
    inner: Arc<Mutex<EventHandlerFunc>>,
}

impl EventHandler {
    pub fn new(handler: EventHandlerFunc) -> Self {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: Update) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

// is public for use with the command framework
#[doc(hidden)]
#[derive(Clone)]
pub struct RawEventHandler {
    inner: Arc<Mutex<RawEventHandlerFunc>>,
}

impl RawEventHandler {
    pub fn new(handler: RawEventHandlerFunc) -> Self {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: RawUpdate) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

// is public for use with the command framework
#[doc(hidden)]
#[derive(Clone)]
pub struct MessageHandler {
    inner: Arc<Mutex<MessageHandlerFunc>>,
}

impl MessageHandler {
    pub fn new(handler: MessageHandlerFunc) -> Self {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: Message) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<MessageHandlerFunc> for MessageHandler {
    fn from(func: MessageHandlerFunc) -> MessageHandler {
        Self::new(func)
    }
}

// is public for use with the command framework
#[doc(hidden)]
#[derive(Clone)]
pub struct InlineQueryHandler {
    inner: Arc<Mutex<InlineQueryHandlerFunc>>,
}

impl InlineQueryHandler {
    pub fn new(handler: InlineQueryHandlerFunc) -> Self {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: InlineQuery) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<InlineQueryHandlerFunc> for InlineQueryHandler {
    fn from(func: InlineQueryHandlerFunc) -> InlineQueryHandler {
        Self::new(func)
    }
}

// is public for use with the command framework
#[doc(hidden)]
#[derive(Clone)]
pub struct InlineResultHandler {
    inner: Arc<Mutex<InlineResultHandlerFunc>>,
}

impl InlineResultHandler {
    pub fn new(handler: InlineResultHandlerFunc) -> Self {
        Self {
            inner: Arc::new(Mutex::new(handler)),
        }
    }

    pub fn call(&self, c: Context, u: ChosenInlineResult) -> FutureOutcome {
        let h = self.clone();
        std::boxed::Box::pin(async move {
            let func = h.inner.lock().await;
            let fut = (func)(c, u);
            fut.await;
        })
    }
}

impl From<InlineResultHandlerFunc> for InlineResultHandler {
    fn from(func: InlineResultHandlerFunc) -> InlineResultHandler {
        Self::new(func)
    }
}
