use super::APIConnector;
use parking_lot::RwLock;
use std::sync::Arc;
use typemap::ShareMap;


#[derive(Clone)]
pub struct Context {
    pub api: Arc<Box<APIConnector>>,
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
