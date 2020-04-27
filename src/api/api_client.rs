use super::{api::API, endpoints::APIEndpoint, response::Response};
use crate::utils::{
    encode_multipart_form_data,
    result::{Result, TelegramError},
    AsFormData,
    FormDataFile,
    BOUNDARY,
};
use async_trait::async_trait;
use hyper::{body::HttpBody, client::HttpConnector, Body, Client, Method, Request};
use std::io::Write;

static TELEGRAM_API: &str = "https://api.telegram.org/bot";

/// A default implementation of the [`API`] trait.
///
/// It requires your bot token in order to interact with the telegram API and
/// also allows you to configure your own [`hyper::Client`] for it to use.
///
/// Using the default `APIClient` is as easy as:
/// ```rust,no_run
/// 
/// use telexide::api::{APIClient};
///
/// let client = APIClient::new_default(token);
/// client.send_message(&message)
/// ```
///
/// In most cases you would want to get updates though and the [`Client`] is
/// best suited for that, as it allows for easier handling of those updates
///
/// [`Client`]: ../client/struct.Client.html
pub struct APIClient {
    hyper_client: Client<hyper_tls::HttpsConnector<HttpConnector>>,
    token: String,
}

impl APIClient {
    /// Creates a new `APIClient` with the provided token and hyper client (if
    /// it is Some).
    pub fn new(
        hyper_client: Option<Client<hyper_tls::HttpsConnector<HttpConnector>>>,
        token: String,
    ) -> Self {
        let client = if let Some(c) = hyper_client {
            c
        } else {
            hyper::Client::builder().build(hyper_tls::HttpsConnector::new())
        };

        Self {
            hyper_client: client,
            token,
        }
    }

    /// Creates a new `APIClient` with the provided token and the default hyper
    /// client.
    pub fn new_default(token: String) -> Self {
        Self {
            hyper_client: hyper::Client::builder().build(hyper_tls::HttpsConnector::new()),
            token,
        }
    }

    fn parse_endpoint(&self, endpoint: &APIEndpoint) -> String {
        format!("{}{}/{}", TELEGRAM_API, self.token, endpoint)
    }

    /// Sends a request to the provided `APIEndpoint` with the data provided
    /// (does not support files)
    pub async fn request<D>(&self, endpoint: APIEndpoint, data: Option<&D>) -> Result<Response>
    where
        D: ?Sized + serde::Serialize,
    {
        let data: Option<serde_json::Value> = if let Some(d) = data {
            Some(serde_json::to_value(d)?)
        } else {
            None
        };

        match endpoint {
            e if e.get_method() == Method::GET => self.get(e, data).await,
            e if e.get_method() == Method::POST => self.post(e, data).await,
            _ => Err(TelegramError::InvalidEndpoint.into()),
        }
    }
}

#[async_trait]
impl API for APIClient {
    async fn get(
        &self,
        endpoint: APIEndpoint,
        data: Option<serde_json::Value>,
    ) -> Result<Response> {
        let req_builder = Request::get(self.parse_endpoint(&endpoint))
            .header("content-type", "application/json")
            .header("accept", "application/json");

        let request = if let Some(d) = data {
            req_builder.body(Body::from(serde_json::to_string(&d)?))?
        } else {
            req_builder.body(Body::empty())?
        };

        let mut response = self.hyper_client.request(request).await?;

        let mut res: Vec<u8> = Vec::new();
        while let Some(chunk) = response.body_mut().data().await {
            res.write_all(&chunk?)?
        }

        Ok(serde_json::from_slice(&res)?)
    }

    async fn post(
        &self,
        endpoint: APIEndpoint,
        data: Option<serde_json::Value>,
    ) -> Result<Response> {
        let req_builder = Request::post(self.parse_endpoint(&endpoint))
            .header("content-type", "application/json")
            .header("accept", "application/json");

        let request = if let Some(d) = data {
            req_builder.body(Body::from(serde_json::to_string(&d)?))?
        } else {
            req_builder.body(Body::empty())?
        };

        let mut response = self.hyper_client.request(request).await?;

        let mut res: Vec<u8> = Vec::new();
        while let Some(chunk) = response.body_mut().data().await {
            res.write_all(&chunk?)?
        }

        Ok(serde_json::from_slice(&res)?)
    }

    async fn post_file(
        &self,
        endpoint: APIEndpoint,
        data: Option<serde_json::Value>,
        files: Option<Vec<FormDataFile>>,
    ) -> Result<Response> {
        if files.is_none() {
            return self.post(endpoint, data).await;
        }

        let mut files = files.expect("no files");
        if files.is_empty() {
            return self.post(endpoint, data).await;
        }

        let req_builder = Request::post(self.parse_endpoint(&endpoint))
            .header(
                "content-type",
                format!("multipart/form-data; boundary={}", BOUNDARY),
            )
            .header("accept", "application/json");

        //files = Vec::new();
        if data.is_some() {
            files.append(&mut data.expect("no data").as_form_data()?)
        }

        let bytes = encode_multipart_form_data(&files)?;
        //println!("data sent: {}", String::from_utf8_lossy(&bytes));
        let request = req_builder.body(Body::from(bytes))?;

        let mut response = self.hyper_client.request(request).await?;

        let mut res: Vec<u8> = Vec::new();
        while let Some(chunk) = response.body_mut().data().await {
            res.write_all(&chunk?)?
        }

        Ok(serde_json::from_slice(&res)?)
    }
}
