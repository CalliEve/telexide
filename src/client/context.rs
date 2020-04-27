use super::APIConnector;
use parking_lot::RwLock;
use std::sync::Arc;
use typemap::ShareMap;

/// The context object is an utility object that gets passed to all event
/// handlers, it provides access to the API client and to any custom data you
/// have set in the data object.
#[derive(Clone)]
pub struct Context {
    /// The API client, implementing the [`API`] trait
    ///
    /// [`API`]: ../api/trait.API.html
    pub api: Arc<Box<APIConnector>>,
    /// A clone of [`Client::data`], see its documentation for more detail
    ///
    /// [`Client::data`]: struct.Client.html#structfield.data
    pub data: Arc<RwLock<ShareMap>>,
}

impl Context {
    pub fn new(api: Arc<Box<APIConnector>>, data: Arc<RwLock<ShareMap>>) -> Self {
        Self {
            api,
            data,
        }
    }
}
