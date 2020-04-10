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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Update {
    pub update_id: i64,
    pub content: UpdateContent,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateContent {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    Unknown,
}

impl From<RawUpdate> for Update {
    fn from(raw: RawUpdate) -> Update {
        let update_id = raw.update_id;
        let make_update = |content: UpdateContent| {
            return Self {
                update_id,
                content,
            };
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
