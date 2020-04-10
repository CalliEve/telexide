use super::{Location, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub inline_message_id: Option<String>,
}
