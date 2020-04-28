use super::types::CommandResult;
use crate::{client::Context, model::Message};
use std::{future::Future, pin::Pin};

pub(crate) type CommandOutcome = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub(crate) type CommandHandlerFunc = fn(Context, Message) -> CommandOutcome;
