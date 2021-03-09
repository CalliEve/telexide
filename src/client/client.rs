use super::{
    APIConnector,
    ClientBuilder,
    Context,
    EventHandlerFunc,
    RawEventHandlerFunc,
    UpdatesStream,
    Webhook,
    WebhookOptions,
};
use crate::{
    api::{
        types::{SetWebhook, UpdateType},
        APIClient,
    },
    framework::Framework,
    model::Update,
    Result,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::sync::Arc;
use typemap::ShareMap;

/// The Client is the main object to manage your interaction with telegram.
///
/// It handles the incoming update objects from telegram and dispatches them to
/// your event handlers and commands, providing those with access to shared data
/// and easy access to the telegram API itself.
///
/// # Event Handlers
///
/// Event handlers can be configured to be called upon every update that is
/// received. (Later on support will be added for subscribing to more specific
/// update events)
///
/// Note that you do not need to manually handle retrieving updates,
/// as they are handled internally and then dispatched to your event handlers.
///
/// # Examples
/// ```rust,no_run
/// use telexide::prelude::*;
///
/// #[prepare_listener]
/// async fn event_listener(ctx: Context, update: Update) {
///     println!("received an update!")
/// }
///
/// #[tokio::main]
/// async fn main() -> telexide::Result<()> {
///     # let token = "test token";
///     let mut client = Client::new(token);
///     client.subscribe_handler_func(event_listener);
///
///     client.start().await
/// }
/// ```
#[derive(Clone)]
pub struct Client {
    /// The API client, it contains all the methods to talk to the telegram api,
    /// more documentation can be found over at the [`API`] docs
    ///
    /// [`API`]: ../api/trait.API.html
    pub api_client: Arc<Box<APIConnector>>,
    /// Your custom data that you want to be shared amongst event handlers and
    /// commands.
    ///
    /// The purpose of the data field is to be accessible and persistent across
    /// contexts; that is, data can be modified by one context, and will persist
    /// through the future and be accessible through other contexts. This is
    /// useful for anything that should "live" through the program: counters,
    /// database connections, custom user caches, etc.
    /// Therefore this ShareMap requires all types it will contain to be Send +
    /// Sync.
    ///
    /// When using a [`Context`], this data will be available as
    /// [`Context::data`].
    ///
    /// Refer to the [repeat_image_bot] example for an example on using the
    /// `data` field
    ///
    /// [repeat_image_bot]: https://github.com/Baev1/telexide/tree/master/examples/repeat_image_bot.rs
    pub data: Arc<RwLock<ShareMap>>,
    pub(super) event_handlers: Vec<EventHandlerFunc>,
    pub(super) raw_event_handlers: Vec<RawEventHandlerFunc>,
    pub(super) framework: Option<Arc<Framework>>,
    pub(super) webhook_opts: Option<WebhookOptions>,
    /// The update types that you want to receive, see the documentation of
    /// [`UpdateType`] for more information
    pub allowed_updates: Vec<UpdateType>,
}

impl Client {
    /// Creates a Client object with default values and no framework
    pub fn new<T: ToString>(token: T) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            framework: None,
            webhook_opts: None,
            allowed_updates: Vec::new(),
        }
    }

    /// Creates a Client object with default values, but with a [`Framework`]
    pub fn with_framework<T: ToString>(fr: Arc<Framework>, token: T) -> Self {
        Self {
            api_client: Arc::new(Box::new(APIClient::new(None, token))),
            event_handlers: Vec::new(),
            raw_event_handlers: Vec::new(),
            data: Arc::new(RwLock::new(ShareMap::custom())),
            webhook_opts: None,
            framework: Some(fr),
            allowed_updates: Vec::new(),
        }
    }

    /// Returns a new `ClientBuilder`
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Starts the client and blocks until an error happens in the updates
    /// stream or the program exits (for example due to a panic).
    /// If using the framework, it will update your commands in telegram.
    /// If using a webhook, it will handle it, else it will use polling using a
    /// default [`UpdatesStream`] object
    pub async fn start(&self) -> Result<()> {
        if let Some(opts) = &self.webhook_opts {
            self.start_with_webhook(opts).await
        } else {
            let mut stream = UpdatesStream::new(self.api_client.clone());
            stream.set_allowed_updates(self.allowed_updates.clone());

            self.start_with_stream(&mut stream).await
        }
    }

    /// Starts the client and blocks until an error happens in the updates
    /// stream or the program exits (for example due to a panic).
    /// If using the framework, it will update your commands in telegram
    /// You have to provide your own [`UpdatesStream`] object
    pub async fn start_with_stream(&self, stream: &mut UpdatesStream) -> Result<()> {
        if let Some(fr) = self.framework.clone() {
            self.api_client
                .set_my_commands(fr.get_commands().into())
                .await?;
        }

        log::info!("starting long polling to listen for updates from telegram api");
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

    /// Starts the client and blocks until an error happens in the webhook
    /// handling or the program exits (for example due to a panic).
    /// If using the framework, it will update your commands in telegram
    /// You have to provide your own [`WebhookOptions`] object
    pub async fn start_with_webhook(&self, opts: &WebhookOptions) -> Result<()> {
        if let Some(fr) = self.framework.clone() {
            self.api_client
                .set_my_commands(fr.get_commands().into())
                .await?;
        }

        if let Some(webhook_url) = &opts.url {
            self.api_client
                .set_webhook(SetWebhook {
                    url: webhook_url.to_string(),
                    certificate: None,
                    max_connections: None,
                    allowed_updates: Some(self.allowed_updates.clone()),
                    drop_pending_updates: None,
                    ip_address: None, // TODO: add opts for these
                })
                .await?;
        }

        log::info!("starting to listen on the webhook");
        let mut receiver = Webhook::new(opts).start();
        while let Some(u) = receiver.recv().await {
            match u {
                Ok(update) => {
                    self.fire_handlers(update);
                },
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    /// Subscribes an update event handler function ([`EventHandlerFunc`]) to
    /// the client and will be ran whenever a new update is received
    pub fn subscribe_handler_func(&mut self, handler: EventHandlerFunc) {
        self.event_handlers.push(handler);
    }

    /// Subscribes a raw update event handler function ([`RawEventHandlerFunc`])
    /// to the client and will be ran whenever a new update is received
    pub fn subscribe_raw_handler(&mut self, handler: RawEventHandlerFunc) {
        self.raw_event_handlers.push(handler);
    }

    // public only for testing purposes
    #[doc(hidden)]
    pub fn fire_handlers(&self, update: Update) {
        for h in self.raw_event_handlers.clone() {
            let ctx = Context::new(self.api_client.clone(), self.data.clone());
            let u = update.clone();
            tokio::spawn(async move { h(ctx, u.into()).await });
        }

        for h in self.event_handlers.clone() {
            let ctx = Context::new(self.api_client.clone(), self.data.clone());
            let u = update.clone();
            tokio::spawn(async move { h(ctx, u).await });
        }

        if self.framework.is_some() {
            let ctx = Context::new(self.api_client.clone(), self.data.clone());
            let fr = self.framework.clone();
            fr.as_ref()
                .expect("Framework needs to be set before trying to fire commands")
                .fire_commands(ctx, update);
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
            webhook_opts: None,
            allowed_updates: Vec::new(),
        }
    }
}
