use serde::{Deserialize, Serialize};
use super::{InputFile, UpdateType};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetWebHook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked.
    /// See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.
    pub certificate: Option<InputFile>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery,
    /// 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server,
    /// and higher values to increase your bot’s throughput.
    pub max_connections: Option<i64>,
    /// List of the update types you want your bot to receive.
    ///
    /// Please note that this parameter doesn't affect updates created before the call to the set_webhook,
    /// so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<UpdateType>>
}
