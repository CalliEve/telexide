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

#[derive(Clone)]
pub struct Client {
    pub(super) api_client: Arc<Box<APIConnector>>,
    pub data: Arc<RwLock<ShareMap>>,
    pub(super) event_handlers: Vec<EventHandler>,
    pub(super) raw_event_handlers: Vec<RawEventHandler>,
    pub(super) framework: Option<Arc<Framework>>,
    pub allowed_updates: Vec<UpdateType>
}

impl Client {
    pub fn new_default_without_framework(token: String) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: None,
            allowed_updates: Vec::new()
        }
    }

    pub fn new_default(fr: Arc<Framework>, token: String) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: Some(fr),
            allowed_updates: Vec::new()
        }
    }

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

    pub fn subscribe_handler(&mut self, handler: EventHandlerFunc)
    {
        self.event_handlers
            .push(EventHandler::new(handler));
    }

    pub fn subscribe_raw_handler(&mut self, handler: RawEventHandlerFunc)
    {
        self.raw_event_handlers
            .push(RawEventHandler::new(handler));
    }

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
