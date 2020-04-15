pub use super::utils::TextBlock;
use super::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum MessageEntity {
    #[serde(rename = "mention")]
    Mention(TextBlock),
    #[serde(rename = "hash_tag")]
    HashTag(TextBlock),
    #[serde(rename = "cash_tag")]
    CashTag(TextBlock),
    #[serde(rename = "bot_command")]
    BotCommand(TextBlock),
    #[serde(rename = "url")]
    Url(TextBlock),
    #[serde(rename = "email")]
    Email(TextBlock),
    #[serde(rename = "phone_number")]
    PhoneNumber(TextBlock),
    #[serde(rename = "bold")]
    Bold(TextBlock),
    #[serde(rename = "italic")]
    Italic(TextBlock),
    #[serde(rename = "underline")]
    Underline(TextBlock),
    #[serde(rename = "strike_through")]
    StrikeThrough(TextBlock),
    #[serde(rename = "code")]
    Code(TextBlock),
    #[serde(rename = "pre")]
    Pre(Pre),
    #[serde(rename = "text_link")]
    TextLink(TextLink),
    #[serde(rename = "text_mention")]
    TextMention(TextMention),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pre {
    #[serde(flatten)]
    pub text_block: TextBlock,
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextLink {
    #[serde(flatten)]
    pub text_block: TextBlock,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextMention {
    #[serde(flatten)]
    pub text_block: TextBlock,
    pub user: User,
}
