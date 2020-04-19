use super::User;
use serde::{Deserialize, Serialize};

/// This object contains basic information about an invoice.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code
    pub currency: String,
    /// Total price in the smallest units of the [currency](https://core.telegram.org/bots/payments#supported-currencies)
    /// (integer, not float). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in
    /// [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    pub total_amount: usize,
}

/// This object contains basic information about a successful payment.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code
    pub currency: String,
    /// Total amount in the smallest units of the [currency](https://core.telegram.org/bots/payments#supported-currencies)
    /// (integer, not float). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in
    /// [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    pub total_amount: usize,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

/// This object represents information about an order.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

/// This object represents a shipping address.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

/// This object contains information about an incoming shipping query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

/// This object contains information about an incoming pre-checkout query.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 [currency](https://core.telegram.org/bots/payments#supported-currencies) code
    pub currency: String,
    /// Total amount in the smallest units of the [currency](https://core.telegram.org/bots/payments#supported-currencies)
    /// (integer, not float). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in
    /// [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    pub total_amount: usize,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
}

/// This object represents one shipping option.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>
}

/// This object represents a portion of the price for goods or services.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the [currency](https://core.telegram.org/bots/payments#supported-currencies)
    /// (integer, not float). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in
    /// [currencies.json](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    pub amount: i64,
}
