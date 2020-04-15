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

        match ref_mut.current_request {
            Some(ref mut req) => match req.as_mut().poll(cx) {
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
            },
            None => {
                ref_mut.poll_telegram();
                return Pin::new(ref_mut).poll_next(cx);
            },
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

    pub fn set_limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_timout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = timeout;
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

    pub fn remove_allowed_updates(&mut self, to_remove: UpdateType) -> &mut Self {
        self.allowed_updates.retain(|t| t != &to_remove);
        self
    }
}
