//! The api module provides the API trait and `APIClient`, providing methods to
//! perform requests to the telegram API

mod api;
mod api_client;
mod endpoints;
mod response;
pub mod types;

pub use api::API;
pub use api_client::APIClient;
pub use endpoints::APIEndpoint;
pub use response::Response;
