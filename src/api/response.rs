use crate::utils::result::{Result, TelegramError};
use serde::{Deserialize, Serialize};

/// The response object that gets returned from the telegram API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<serde_json::Value>,
}

impl<T> From<Response> for Result<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from(resp: Response) -> Result<T> {
        if resp.ok {
            Ok(serde_json::from_value(resp.result.ok_or_else(|| {
                TelegramError::Unknown("response had no result".to_owned())
            })?)?)
        } else if resp.description.is_some() {
            Err(TelegramError::APIResponseError(
                resp.description
                    .unwrap_or_else(|| "api error does not contain description".to_owned()),
            )
            .into())
        } else {
            Err(TelegramError::Unknown(
                "got error without description from the telegram api".to_owned(),
            )
            .into())
        }
    }
}
