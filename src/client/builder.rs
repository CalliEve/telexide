use super::{APIConnector, Client, EventHandler, event_handlers::EventHandlerFunc};
use crate::{api::{APIClient, types::UpdateType}, framework::Framework};

use parking_lot::RwLock;
use std::sync::Arc;
use typemap::ShareMap;

pub struct ClientBuilder {
    hyper_client: Option<hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>>,
    api_client: Option<Arc<Box<APIConnector>>>,
    webhook: Option<String>,
    framework: Option<Arc<Framework>>,
    token: Option<String>,
    allowed_updates: Vec<UpdateType>,
    event_handlers: Vec<EventHandler>,
}

impl ClientBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            api_client: None,
            hyper_client: None,
            webhook: None,
            framework: None,
            token: None,
            allowed_updates: Vec::new(),
            event_handlers: Vec::new()
        }
    }

    pub fn set_webhook(&mut self, webhook: String) -> &mut Self {
        self.webhook = Some(webhook);
        self
    }

    pub fn set_framework(&mut self, framework: Arc<Framework>) -> &mut Self {
        self.framework = Some(framework);
        self
    }

    pub fn set_token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn set_hyper_client(
        &mut self,
        client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    ) -> &mut Self {
        self.hyper_client = Some(client);
        self
    }

    pub fn set_api_client(&mut self, client: Arc<Box<APIConnector>>) -> &mut Self {
        self.api_client = Some(client);
        self
    }

    pub fn set_allowed_updates(&mut self, allowed: Vec<UpdateType>) -> &mut Self {
        self.allowed_updates = allowed;
        self
    }

    pub fn add_allowed_updates(&mut self, allowed: UpdateType) -> &mut Self {
        self.allowed_updates.push(allowed);
        self
    }

    pub fn remove_allowed_updates(&mut self, denied: UpdateType) -> &mut Self {
        self.allowed_updates.retain(|t| *t != denied);
        self
    }

    pub fn add_handler(&mut self, handler: EventHandlerFunc) -> &mut Self {
        self.event_handlers.push(EventHandler::new(handler));
        self
    }

    pub fn build(&self) -> Client {
        if let Some(c) = self.api_client.clone() {
            Client {
                api_client: c,
                event_handlers: self.event_handlers.clone(),
                raw_event_handlers: Vec::new(),
                data: Arc::new(RwLock::new(ShareMap::custom())),
                framework: self.framework.clone(),
                allowed_updates: self.allowed_updates.clone(),
            }
        } else {
            Client {
                api_client: Arc::new(Box::new(APIClient::new(
                    self.hyper_client.clone(),
                    self.token
                        .clone()
                        .expect("A token must be provided for the telegram bot to work"),
                ))),
                event_handlers: self.event_handlers.clone(),
                raw_event_handlers: Vec::new(),
                data: Arc::new(RwLock::new(ShareMap::custom())),
                framework: self.framework.clone(),
                allowed_updates: self.allowed_updates.clone(),
            }
        }
    }
}
