use crate::client::Context;
use crate::model::Message;
use std::pin::Pin;
use std::future::Future;
use super::types::CommandResult;

pub(crate) type CommandOutcome = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub(crate) type CommandHandlerFunc = fn(Context, Message) -> CommandOutcome;
