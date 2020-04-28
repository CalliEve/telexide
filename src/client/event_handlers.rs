use super::{Context, FutureOutcome};
use crate::model::{raw::RawUpdate, Update};

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
