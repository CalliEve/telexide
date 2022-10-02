pub use super::utils::TextBlock;
use super::User;
use serde::{Deserialize, Serialize};

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum MessageEntity {
    /// A mention (`@username`)
    #[serde(rename = "mention")]
    Mention(TextBlock),
    /// A hashtag (`#hashtag`)
    #[serde(rename = "hash_tag")]
    HashTag(TextBlock),
    /// A cashtag (`$USD`)
    #[serde(rename = "cash_tag")]
    CashTag(TextBlock),
    /// A bot command (`/start@bot_name`)
    #[serde(rename = "bot_command")]
    BotCommand(TextBlock),
    /// An url (`https://telegram.org`)
    #[serde(rename = "url")]
    Url(TextBlock),
    /// An email address (`do-not-reply@telegram.org`)
    #[serde(rename = "email")]
    Email(TextBlock),
    /// A phone number (`+1-212-555-0123`)
    #[serde(rename = "phone_number")]
    PhoneNumber(TextBlock),
    /// Bold text
    #[serde(rename = "bold")]
    Bold(TextBlock),
    /// Italic text
    #[serde(rename = "italic")]
    Italic(TextBlock),
    /// Underlined text
    #[serde(rename = "underline")]
    Underline(TextBlock),
    /// strikethrough text
    #[serde(rename = "strikethrough")]
    StrikeThrough(TextBlock),
    /// spoiler message
    #[serde(rename = "spoiler")]
    Spoiler(TextBlock),
    /// A monowidth code string
    #[serde(rename = "code")]
    Code(TextBlock),
    /// a monowidth code block
    #[serde(rename = "pre")]
    Pre(Pre),
    /// A clickable text URL
    #[serde(rename = "text_link")]
    TextLink(TextLink),
    /// A mention of users without usernames
    #[serde(rename = "text_mention")]
    TextMention(TextMention),
    /// Inline custom emoji stickers
    #[serde(rename = "custom_emoji")]
    CustomEmoji(InlineCustomEmoji),
}

/// A monowidth code block
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pre {
    /// The part of the text that is the code block
    #[serde(flatten)]
    pub text_block: TextBlock,
    /// The programming language of the entity text
    pub language: String,
}

/// A clickable text URL
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextLink {
    /// The part of the text that if clicked will open an url
    #[serde(flatten)]
    pub text_block: TextBlock,
    /// The url that will be opened after user taps on the text
    pub url: String,
}

/// For users [without usernames](https://telegram.org/blog/edit#new-mentions)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextMention {
    /// The part of the text that is the mention
    #[serde(flatten)]
    pub text_block: TextBlock,
    /// The mentioned user
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InlineCustomEmoji {
    /// The part of the text that is the custom emoji
    #[serde(flatten)]
    pub text_block: TextBlock,
    /// For “custom_emoji” only, unique identifier of the custom emoji. Use [`get_custom_emoji_stickers`] to get full information about the sticker
    ///
    /// [`get_custom_emoji_stickers`]: ../../api/trait.API.html#method.get_custom_emoji_stickers
    pub custom_emoji_id: String,
}
