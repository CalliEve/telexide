use super::{InputFile, UpdateType};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call [`set_webhook`]
///
/// **note:** having an webhook url set prevents you from using [`get_updates`]
///
/// [`get_updates`]: ../../api/trait.API.html#method.get_updates
/// [`set_webhook`]: ../../api/trait.API.html#method.set_webhook
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook
    /// integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.
    pub certificate: Option<InputFile>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery, 1-100. Defaults to 40. Use lower values to
    /// limit the load on your bot‘s server, and higher values to increase
    /// your bot’s throughput.
    pub max_connections: Option<i64>,
    /// List of the update types you want your bot to receive.
    ///
    /// Please note that this parameter doesn't affect updates created before
    /// the call to the set_webhook, so unwanted updates may be received for
    /// a short period of time.
    pub allowed_updates: Option<Vec<UpdateType>>,
    /// The fixed IP address which will be used to send webhook requests instead
    /// of the IP address resolved through DNS
    pub ip_address: Option<String>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

/// Struct for holding data needed to call [`delete_webhook`]
///
/// [`delete_webhook`]: ../../api/trait.API.html#method.delete_webhook
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DeleteWebhook {
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}
