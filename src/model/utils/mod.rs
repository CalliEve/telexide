use serde::{Deserialize, Serialize};

pub mod text;
pub mod unix_date_formatting;

pub use text::TextBlock;

/// Can be a string or an integer. Often used for the id of a chat, as that can
/// also be the username of a supergroup.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum IntegerOrString {
    Integer(i64),
    String(String),
}

impl From<i64> for IntegerOrString {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<String> for IntegerOrString {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}
