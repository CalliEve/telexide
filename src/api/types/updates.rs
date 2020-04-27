use serde::{Deserialize, Serialize};

/// struct for holding data needed to call
/// [`get_updates`]
///
/// [`get_updates`]:
/// ../../api/trait.API.html#method.get_updates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<UpdateType>>,
}

impl GetUpdates {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn set_offset(&mut self, offset: i64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn set_limit(&mut self, mut limit: usize) -> &mut Self {
        if limit > 100 {
            limit = 100;
        }
        self.limit = Some(limit);
        self
    }

    pub fn set_timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn set_allowed_updates(&mut self, allowed_updates: Vec<UpdateType>) -> &mut Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    pub fn add_allowed_updates(&mut self, allowed_update: UpdateType) -> &mut Self {
        if let Some(ref mut a) = self.allowed_updates {
            a.push(allowed_update)
        } else {
            self.allowed_updates = Some(vec![allowed_update])
        }
        self
    }
}

impl std::default::Default for GetUpdates {
    fn default() -> Self {
        Self::new()
    }
}

/// The type of an update, can be used for specifying which update types you
/// want to receive
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UpdateType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "edited_message")]
    EditedMessage,
    #[serde(rename = "channel_post")]
    ChannelPost,
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost,
    #[serde(rename = "inline_query")]
    InlineQuery,
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult,
    #[serde(rename = "callback_query")]
    CallbackQuery,
    #[serde(rename = "shipping_query")]
    ShippingQuery,
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "poll_answer")]
    PollAnswer,
}
