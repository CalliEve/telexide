use super::{APIConnector, Context, EventHandler, RawEventHandler, event_handlers::{EventHandlerFunc, RawEventHandlerFunc}, UpdatesStream};
use crate::{
    api::{APIClient, types::UpdateType},
    framework::Framework,
    model::{raw::RawUpdate, Update},
    Result,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::sync::Arc;
use typemap::ShareMap;

/// The main object to manage your interaction with telegram
#[derive(Clone)]
pub struct Client {
    /// The API client, it contains all the methods to talk to the telegram api
    pub api_client: Arc<Box<APIConnector>>,
    /// Your custom data that you want to be shared amongst event handlers and commands
    pub data: Arc<RwLock<ShareMap>>,
    pub(super) event_handlers: Vec<EventHandler>,
    pub(super) raw_event_handlers: Vec<RawEventHandler>,
    pub(super) framework: Option<Arc<Framework>>,
    /// The update types that you want to receive
    pub allowed_updates: Vec<UpdateType>
}

impl Client {
    /// Creates a Client object with default values and no framework
    pub fn new(token: String) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: None,
            allowed_updates: Vec::new()
        }
    }

    /// Creates a Client object with default values, but with a framework
    pub fn with_framework(fr: Arc<Framework>, token: String) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: Some(fr),
            allowed_updates: Vec::new()
        }
    }

    /// Starts the client and blocks until an error happens in the updates stream or the program exits (for example due to a panic).
    /// If using the framework, it will update your commands in telegram
    /// This uses a default UpdatesStream object
    pub async fn start(&self) -> Result<()> {
        if let Some(fr) = self.framework.clone() {
            self.api_client
                .set_my_commands(fr.get_commands().into())
                .await?;
        }

        let mut stream = UpdatesStream::new(self.api_client.clone());
        stream.set_allowed_updates(self.allowed_updates.clone());

        while let Some(poll) = stream.next().await {
            match poll {
                Ok(update) => {
                    self.fire_handlers(update);
                },
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    /// Starts the client and blocks until an error happens in the updates stream or the program exits (for example due to a panic).
    /// If using the framework, it will update your commands in telegram
    /// You have to provide your own UpdatesStream object
    pub async fn start_with_stream(&self, stream: &mut UpdatesStream) -> Result<()> {
        if let Some(fr) = self.framework.clone() {
            self.api_client
                .set_my_commands(fr.get_commands().into())
                .await?;
        }

        while let Some(poll) = stream.next().await {
            match poll {
                Ok(update) => {
                    self.fire_handlers(update);
                },
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    /// Subscribes an update event handler to the client and will be ran when a new update is received
    pub fn subscribe_handler(&mut self, handler: EventHandlerFunc)
    {
        self.event_handlers
            .push(EventHandler::new(handler));
    }

//    /// Subscribes a raw update event handler to the client and will be ran when a new update is received
//    pub fn subscribe_raw_handler(&mut self, handler: RawEventHandlerFunc)
//    {
//        self.raw_event_handlers
//            .push(RawEventHandler::new(handler));
//    }

    /// for testing purposes
    #[doc(hidden)]
    pub fn fire_handlers(&self, update: Update) {
        for h in self.event_handlers.clone() {
            let ctx = Context::new(self.api_client.clone(), self.data.clone());
            let u = update.clone();
            tokio::spawn(async move { h.call(
                ctx,
                u,
            ).await});
        }

        if self.framework.is_some() {
            let ctx = Context::new(self.api_client.clone(), self.data.clone());
            let fr = self.framework.clone();
            Framework::fire_commands(
                fr.as_ref()
                    .expect("Framework needs to be set before trying to fire commands")
                    .to_owned(),
                ctx,
                update,
            )
        }
    }
}

impl From<Box<APIConnector>> for Client {
    fn from(api: Box<APIConnector>) -> Self {
        Self {
            api_client: Arc::new(api),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: None,
            allowed_updates: Vec::new(),
        }
    }
}
