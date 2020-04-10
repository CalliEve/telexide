use crate::{
    model::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove},
    prelude::Message,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: bool,
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum ParseMode {
    MarkdownV2,
    Markdown,
    HTML,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl SendMessage {
    pub fn new(chat_id: i64, text: &str) -> Self {
        Self {
            chat_id,
            text: text.to_owned(),
            parse_mode: None,
            disable_notification: false,
            disable_web_page_preview: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn set_parse_mode(&mut self, mode: &ParseMode) -> &mut Self {
        self.parse_mode = Some(mode.to_owned());
        self
    }

    pub fn reply_to_message(&mut self, message: &Message) -> &mut Self {
        self.reply_to_message_id = Some(message.message_id);
        self
    }

    pub fn set_reply_to_message_id(&mut self, id: i64) -> &mut Self {
        self.reply_to_message_id = Some(id);
        self
    }

    pub fn set_reply_markup(&mut self, markup: &ReplyMarkup) -> &mut Self {
        self.reply_markup = Some(markup.to_owned());
        self
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn disable_web_page_preview(&mut self) -> &mut Self {
        self.disable_web_page_preview = true;
        self
    }
}
