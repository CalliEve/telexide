use super::{Context, FutureOutcome};
use crate::model::{raw::RawUpdate, Update};

/// A function that handles a new update, it receives a [`Context`] and
/// [`Update`] and returns a pinned future. Wrap an async function with
/// `#[prepare_listener]` for easier development.
pub type EventHandlerFunc = fn(Context, Update) -> FutureOutcome;

/// A function that handles a new raw update, it receives a [`Context`] and
/// [`RawUpdate`] and returns a pinned future. Wrap an async function with
/// `#[prepare_listener]` for easier development.
pub type RawEventHandlerFunc = fn(Context, RawUpdate) -> FutureOutcome;
