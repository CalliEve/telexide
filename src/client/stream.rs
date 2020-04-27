use futures::{Future, Stream};
use std::{
    cmp::max,
    collections::VecDeque,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use super::APIConnector;
use crate::{
    api::types::{GetUpdates, UpdateType},
    model::Update,
    Result,
};

type FutureUpdate = Pin<Box<dyn Future<Output = Result<Vec<Update>>>>>;

/// The stream of incoming updates, created by long polling the telegram API
/// using their getUpdates endpoint.
///
/// In most use-cases, this will be handled for you by the [`Client`]
/// and the new updates then dispatched to your eventhandlers.
///
/// ## Example
/// ```rust,no_run
/// use std::sync::Arc;
/// use telexide::{
///     api::APIClient,
///     client::UpdatesStream
/// };
///
/// let mut stream = UpdatesStream::new(
///     Arc::new(
///         Box::new(
///             APIClient::new_default(your_token)
///         )
///     )
/// );
///
/// while let Some(poll) = stream.next().await {
///     match poll {
///         Ok(update) => {
///             fire_update_handlers(update);
///         },
///         Err(err) => return Err(err),
///     }
/// }
/// ```
///
/// [`Client`]: struct.Client.html
#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    api: Arc<Box<APIConnector>>,
    buffer: VecDeque<Update>,
    allowed_updates: Vec<UpdateType>,
    offset: i64,
    limit: usize,
    timeout: usize,
    current_request: Option<FutureUpdate>,
}

impl Stream for UpdatesStream {
    type Item = Result<Update>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let ref_mut = self.get_mut();

        if let Some(u) = ref_mut.buffer.pop_front() {
            return Poll::Ready(Some(Ok(u)));
        }

        if let Some(ref mut request) = ref_mut.current_request {
            match request.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(ref res)) if res.is_empty() => {
                    ref_mut.poll_telegram();
                    return Pin::new(ref_mut).poll_next(cx);
                },
                Poll::Ready(Ok(res)) => {
                    for u in res {
                        ref_mut.offset = max(u.update_id, ref_mut.offset);
                        ref_mut.buffer.push_back(u);
                    }
                },
                Poll::Ready(Err(err)) => {
                    ref_mut.poll_telegram();
                    return Poll::Ready(Some(Err(err)));
                },
            };
        } else {
            ref_mut.poll_telegram();
            return Pin::new(ref_mut).poll_next(cx);
        }

        ref_mut.current_request = None;
        Pin::new(ref_mut).poll_next(cx)
    }
}

impl UpdatesStream {
    fn poll_telegram(&mut self) {
        let mut data = GetUpdates::new();
        data.set_limit(self.limit)
            .set_allowed_updates(self.allowed_updates.clone())
            .set_offset(self.offset + 1)
            .set_timeout(self.timeout);

        let api = self.api.clone();
        self.current_request = Some(Box::pin(async move { api.get_updates(data).await }));
    }

    /// creates a new update stream using the provided [`API`]
    ///
    /// [`API`]: ../api/trait.API.html
    pub fn new(api: Arc<Box<APIConnector>>) -> Self {
        Self {
            api,
            buffer: VecDeque::new(),
            allowed_updates: Vec::new(),
            offset: 0,
            limit: 100,
            timeout: 5,
            current_request: None,
        }
    }

    /// Sets the maximum amount of updates retrieved in one API call
    pub fn set_limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Set the timeout in seconds for long polling. Defaults to 5.
    /// Should be positive, short polling should be used for testing purposes
    /// only.
    pub fn set_timout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Set which update types you want to receive
    pub fn set_allowed_updates(&mut self, allowed: Vec<UpdateType>) -> &mut Self {
        self.allowed_updates = allowed;
        self
    }

    /// Add an update type to the list of update types you want to receive
    pub fn add_allowed_updates(&mut self, allowed: UpdateType) -> &mut Self {
        self.allowed_updates.push(allowed);
        self
    }

    /// Remove an update type from the list of update types you want to receive
    pub fn remove_allowed_updates(&mut self, to_remove: &UpdateType) -> &mut Self {
        self.allowed_updates.retain(|t| t != to_remove);
        self
    }
}
