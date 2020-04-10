use super::{types::*, APIEndpoint, response::Response};
use crate::{model::*, utils::result::Result};
use async_trait::async_trait;

#[async_trait]
pub trait API {
    async fn get(&self, endpoint: APIEndpoint, data: Option<serde_json::Value>) -> Result<Response>;

    async fn post(&self, endpoint: APIEndpoint, data: Option<serde_json::Value>)
        -> Result<Response>;

    async fn get_updates(&self, data: GetUpdates) -> Result<Vec<Update>> {
        self
            .get(APIEndpoint::GetUpdates, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    async fn send_message(&self, data: SendMessage) -> Result<Message> {
        self
            .post(APIEndpoint::SendMessage, Some(serde_json::to_value(data)?))
            .await?
            .into()
    }

    async fn set_my_commands(&self, data: SetMyCommands) -> Result<bool> {
        self
            .post(
                APIEndpoint::SetMyCommands,
                Some(serde_json::to_value(data)?),
            )
            .await?
            .into()
    }

    async fn get_my_commands(&self) -> Result<Vec<BotCommand>> {
        self
            .get(APIEndpoint::GetMyCommands, None)
            .await?
            .into()
    }
}
