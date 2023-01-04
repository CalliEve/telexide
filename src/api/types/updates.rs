use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`get_updates`]
///
/// [`get_updates`]:
/// ../../api/trait.API.html#method.get_updates
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    #[must_use]
    pub fn add_allowed_updates(mut self, allowed_update: UpdateType) -> Self {
        if let Some(ref mut a) = self.allowed_updates {
            a.push(allowed_update);
        } else {
            self.allowed_updates = Some(vec![allowed_update]);
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
