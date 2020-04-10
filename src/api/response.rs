use serde::{Deserialize, Serialize};
use crate::utils::result::{Result, TelegramError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<serde_json::Value>,
}

impl Response {
    pub fn parse<T>(self) -> Result<T>
        where T: serde::de::DeserializeOwned
    {
        Ok(serde_json::from_value(self.result.ok_or(TelegramError::Unknown("response had no result".to_owned()))?)?)
    }
}

impl<T> From<Response> for Result<T>
    where T: serde::de::DeserializeOwned
{
    fn from(resp: Response) -> Result<T> {
        if resp.ok {
            resp.parse()
        } else {
            if resp.description.is_some() {
                Err(TelegramError::APIResponseError(resp.description.expect("api error does not contain description")).into())
            } else {
                Err(TelegramError::Unknown("got error without description from the telegram api".to_owned()).into())
            }
        }
    }
}
