use super::{APIConnector, Client, EventHandlerFunc, RawEventHandlerFunc, WebhookOptions};
use crate::{
    api::{types::UpdateType, APIClient, TlsClient},
    framework::Framework,
};

use parking_lot::RwLock;
use std::sync::Arc;
use typemap_rev::TypeMap;

/// A builder for the [`Client`] object to make customisation easier
pub struct ClientBuilder {
    hyper_client: Option<TlsClient>,
    api_client: Option<Arc<Box<APIConnector>>>,
    webhook: Option<WebhookOptions>,
    framework: Option<Arc<Framework>>,
    token: Option<String>,
    allowed_updates: Vec<UpdateType>,
    event_handler_funcs: Vec<EventHandlerFunc>,
    raw_event_handler_funcs: Vec<RawEventHandlerFunc>,
}

impl ClientBuilder {
    /// Creates a bare builder
    // Providing a default gives the impression that is enough, but it is not
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            api_client: None,
            hyper_client: None,
            webhook: None,
            framework: None,
            token: None,
            allowed_updates: Vec::new(),
            event_handler_funcs: Vec::new(),
            raw_event_handler_funcs: Vec::new(),
        }
    }

    /// sets the webhook url for the [`Client`] to listen to
    pub fn set_webhook(&mut self, webhook: &WebhookOptions) -> &mut Self {
        self.webhook = Some(webhook.clone());
        self
    }

    /// Sets the framework for your bot to use, please use the
    /// [`create_framework`] macro for creating it
    ///
    /// [`create_framework`]: ../macro.create_framework.html
    pub fn set_framework(&mut self, framework: Arc<Framework>) -> &mut Self {
        self.framework = Some(framework);
        self
    }

    /// Sets the token to be used in authorizing the API requests of your bot
    #[allow(clippy::needless_pass_by_value)] // Otherwise string literals don't work
    pub fn set_token(&mut self, token: impl ToString) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }

    /// Sets the custom hyper client for the `APIClient` to use
    pub fn set_hyper_client(&mut self, client: TlsClient) -> &mut Self {
        self.hyper_client = Some(client);
        self
    }

    /// Sets the custom API client
    pub fn set_api_client(&mut self, client: Arc<Box<APIConnector>>) -> &mut Self {
        self.api_client = Some(client);
        self
    }

    /// Set the list of update types you want your update handlers to handle
    /// An empty list means all updates *except* `ChatMember`
    pub fn set_allowed_updates(&mut self, allowed: Vec<UpdateType>) -> &mut Self {
        self.allowed_updates = allowed;
        self
    }

    /// Add an update type to the list of update types you want your update
    /// handlers to handle
    ///
    /// An empty list means all updates *except* `ChatMember`
    pub fn add_allowed_updates(&mut self, allowed: UpdateType) -> &mut Self {
        self.allowed_updates.push(allowed);
        self
    }

    /// Remove an update type from the list of update types you want your update
    /// handlers to handle
    ///
    /// Note: An empty list means all updates *except* `ChatMember`
    pub fn remove_allowed_updates(&mut self, denied: &UpdateType) -> &mut Self {
        self.allowed_updates.retain(|t| t != denied);
        self
    }

    /// Adds an [`EventHandlerFunc`] function for handling incoming updates
    pub fn add_handler_func(&mut self, handler: EventHandlerFunc) -> &mut Self {
        self.event_handler_funcs.push(handler);
        self
    }

    /// Adds an [`RawEventHandlerFunc`] function for handling incoming updates
    pub fn add_raw_handler_func(&mut self, handler: RawEventHandlerFunc) -> &mut Self {
        self.raw_event_handler_funcs.push(handler);
        self
    }

    /// Creates the [`Client`] object from the settings set in the
    /// [`ClientBuilder`] object
    pub fn build(&mut self) -> Client {
        if self.framework.is_some()
            && !self.allowed_updates.is_empty()
            && !self.allowed_updates.contains(&UpdateType::Message)
        {
            self.allowed_updates.push(UpdateType::Message);
        }

        self.api_client.clone().map_or_else(
            || Client {
                api_client: Arc::new(Box::new(APIClient::new(
                    self.hyper_client.clone(),
                    self.token
                        .as_ref()
                        .expect("A token must be provided for the telegram bot to work"),
                ))),
                event_handlers: self.event_handler_funcs.clone(),
                raw_event_handlers: self.raw_event_handler_funcs.clone(),
                data: Arc::new(RwLock::new(TypeMap::custom())),
                framework: self.framework.clone(),
                webhook_opts: self.webhook.clone(),
                allowed_updates: self.allowed_updates.clone(),
            },
            |c| Client {
                api_client: c,
                event_handlers: self.event_handler_funcs.clone(),
                webhook_opts: self.webhook.clone(),
                raw_event_handlers: self.raw_event_handler_funcs.clone(),
                data: Arc::new(RwLock::new(TypeMap::custom())),
                framework: self.framework.clone(),
                allowed_updates: self.allowed_updates.clone(),
            },
        )
    }
}
