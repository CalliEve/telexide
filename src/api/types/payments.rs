use crate::model::{LabeledPrice, ReplyMarkup, ShippingOption};
use serde::{Deserialize, Serialize};

/// struct for holding data needed to call
/// [`send_invoice`]
///
/// [`send_invoice`]:
/// ../../api/trait.API.html#method.send_invoice
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub chat_id: i64,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes.
    /// This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payments provider token, obtained via [Botfather](https://t.me/botfather)
    pub provider_token: String,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not
    /// float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See
    /// the exp parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point
    /// for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A vec of suggested amounts of tips in the smallest units of the currency (integer, not
    /// float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts
    /// must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will
    /// have a Pay button, allowing multiple users to pay directly from the forwarded message,
    /// using the same invoice. If non-empty, forwarded copies of the sent message will have a URL
    /// button with a deep link to the bot (instead of a Pay button), with the value used as the
    /// start parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax,
    /// discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// JSON-encoded data about the invoice, which will be shared with the
    /// payment provider. A detailed description of required fields should
    /// be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice.
    /// Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True, if you require the user's full name to complete the order
    pub need_name: bool,
    /// Pass True, if you require the user's phone number to complete the order
    pub need_phone_number: bool,
    /// Pass True, if you require the user's email address to complete the order
    pub need_email: bool,
    /// Pass True, if you require the user's shipping address to complete the
    /// order
    pub need_shipping_address: bool,
    /// Pass True, if user's phone number should be sent to provider
    pub send_phone_number_to_provider: bool,
    /// Pass True, if user's email address should be sent to provider
    pub send_email_to_provider: bool,
    /// Pass True, if the final price depends on the shipping method
    pub is_flexible: bool,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: bool,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Additional interface options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// struct for holding data needed to call
/// [`answer_shipping_query`]
///
/// [`answer_shipping_query`]:
/// ../../api/trait.API.html#method.answer_shipping_query
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Specify True if delivery to the specified address is possible and
    /// False if there are any problems (for example, if delivery to the
    /// specified address is not possible)
    pub ok: bool,
    /// Required if ok is True.
    /// A vec of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if ok is False.
    /// Error message in human readable form that explains why it is impossible
    /// to complete the order (e.g. "Sorry, delivery to your desired address
    /// is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// struct for holding data needed to call
/// [`answer_pre_checkout_query`]
///
/// [`answer_pre_checkout_query`]:
/// ../../api/trait.API.html#method.answer_pre_checkout_query
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.)
    /// and the bot is ready to proceed with the order. Use False if there are
    /// any problems.
    pub ok: bool,
    /// Required if ok is False. Error message in human readable form that
    /// explains the reason for failure to proceed with the checkout (e.g.
    /// "Sorry, somebody just bought the last of our amazing black T-shirts
    /// while you were busy filling out your payment details. Please choose
    /// a different color or garment!"). Telegram will display this message
    /// to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
