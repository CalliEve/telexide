use super::{
    message::RawMessage,
    CallbackQuery,
    ChosenInlineResult,
    InlineQuery,
    Message,
    Poll,
    PollAnswer,
    PreCheckoutQuery,
    ShippingQuery,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The raw update, for most usages the [`Update`] object is easier to use
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawUpdate {
    pub update_id: i64,
    pub message: Option<RawMessage>,
    pub edited_message: Option<RawMessage>,
    pub channel_post: Option<RawMessage>,
    pub edited_channel_post: Option<RawMessage>,
    pub inline_query: Option<InlineQuery>,
    pub chosen_inline_result: Option<ChosenInlineResult>,
    pub callback_query: Option<CallbackQuery>,
    pub shipping_query: Option<ShippingQuery>,
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    pub poll: Option<Poll>,
    pub poll_answer: Option<PollAnswer>,
}

/// This object represents an incoming update
#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially.
    /// This ID becomes especially handy if you’re using Webhooks,
    /// since it allows you to ignore repeated updates or to restore the correct update sequence,
    /// should they get out of order. If there are no new updates for at least a week,
    /// then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// The content of the incoming update
    pub content: UpdateContent,
}

/// The content of an [`Update`]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateContent {
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),
    /// New version of a channel post that is known to the bot and was edited
    EditedChannelPost(Message),
    /// New incoming inline query
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    /// Please see the telegram documentation on the [feedback collecting] for details on how to enable these updates for your bot.
    ///
    /// [feedback collecting]: https://core.telegram.org/bots/inline#collecting-feedback
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    Poll(Poll),
    /// An user changed their answer in a non-anonymous poll.
    /// Bots receive new votes only in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),
    /// An unknown update content
    Unknown,
}

impl From<RawUpdate> for Update {
    fn from(raw: RawUpdate) -> Update {
        let update_id = raw.update_id;
        let make_update = |content: UpdateContent| {
            Self {
                update_id,
                content,
            }
        };

        macro_rules! set_content {
            ($data:expr, $kind:ident) => {
                if let Some(c) = $data {
                    return make_update(UpdateContent::$kind(c.into()));
                }
            };
        }

        set_content!(raw.message, Message);
        set_content!(raw.edited_message, EditedMessage);
        set_content!(raw.channel_post, ChannelPost);
        set_content!(raw.edited_channel_post, EditedChannelPost);
        set_content!(raw.inline_query, InlineQuery);
        set_content!(raw.chosen_inline_result, ChosenInlineResult);
        set_content!(raw.callback_query, CallbackQuery);
        set_content!(raw.shipping_query, ShippingQuery);
        set_content!(raw.pre_checkout_query, PreCheckoutQuery);
        set_content!(raw.poll, Poll);
        set_content!(raw.poll_answer, PollAnswer);

        make_update(UpdateContent::Unknown)
    }
}

impl From<Update> for RawUpdate {
    fn from(update: Update) -> RawUpdate {
        let mut ret = Self {
            update_id: update.update_id,
            message: None,
            edited_message: None,
            channel_post: None,
            edited_channel_post: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
            shipping_query: None,
            pre_checkout_query: None,
            poll: None,
            poll_answer: None,
        };

        match update.content {
            UpdateContent::Message(c) => {
                ret.message = Some(c.into());
                ret
            },
            UpdateContent::EditedMessage(c) => {
                ret.edited_message = Some(c.into());
                ret
            },
            UpdateContent::ChannelPost(c) => {
                ret.channel_post = Some(c.into());
                ret
            },
            UpdateContent::EditedChannelPost(c) => {
                ret.edited_channel_post = Some(c.into());
                ret
            },
            UpdateContent::InlineQuery(c) => {
                ret.inline_query = Some(c);
                ret
            },
            UpdateContent::ChosenInlineResult(c) => {
                ret.chosen_inline_result = Some(c);
                ret
            },
            UpdateContent::CallbackQuery(c) => {
                ret.callback_query = Some(c);
                ret
            },
            UpdateContent::ShippingQuery(c) => {
                ret.shipping_query = Some(c);
                ret
            },
            UpdateContent::PreCheckoutQuery(c) => {
                ret.pre_checkout_query = Some(c);
                ret
            },
            UpdateContent::Poll(c) => {
                ret.poll = Some(c);
                ret
            },
            UpdateContent::PollAnswer(c) => {
                ret.poll_answer = Some(c);
                ret
            },
            _ => ret,
        }
    }
}

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Update, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawUpdate = Deserialize::deserialize(deserializer)?;

        Ok(raw.into())
    }
}

impl Serialize for Update {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawUpdate::from(self.to_owned()).serialize(serializer)
    }
}
