use std::{
    convert::Infallible,
    io::Write,
    net::{IpAddr, SocketAddr},
};

use crate::{
    model::Update,
    utils::result::{Result as TelegramResult, TelegramError},
};
use hyper::{
    body::HttpBody,
    service::{make_service_fn, service_fn},
    Body,
    Method,
    Request,
    Response,
    Server,
    StatusCode,
    Uri,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// Handles listening to the telegram webhook and will provide you with the
/// incoming updates
#[derive(Debug)]
pub struct Webhook {
    opts: WebhookOptions,
}

impl Webhook {
    /// creates a new `Webhook` based on the provided `WebhookOptions`
    pub fn new(opts: &WebhookOptions) -> Self {
        Self {
            opts: opts.clone(),
        }
    }

    /// starts the webhandling and returns a [`Receiver`], which will allow you
    /// to receive the incoming updates
    pub fn start(self) -> Receiver<TelegramResult<Update>> {
        let (tx, rx) = channel(1000);

        tokio::spawn(start_ws(self.opts, tx));
        rx
    }
}

async fn handle_update(
    payload: HandlingPayload,
    req: Request<Body>,
) -> TelegramResult<Response<Body>> {
    let mut response = Response::new(Body::empty());

    let mut raw_body = req.into_body();
    let mut body: Vec<u8> = Vec::new();
    while let Some(chunk) = raw_body.data().await {
        body.write_all(&chunk?)?
    }

    let update: Update = serde_json::from_slice(&body)?;
    let send_res = payload.chan.send(Ok(update)).await;
    if send_res.is_err() {
        return Err(TelegramError::WebhookError.into());
    }

    *response.status_mut() = StatusCode::OK;
    Ok(response)
}

async fn handle_req(
    payload: HandlingPayload,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::POST, path) if path == payload.path => {
            let result = handle_update(payload, req).await;

            if result.is_err() {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            } else {
                response = result.unwrap()
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    }

    Ok(response)
}

async fn start_ws(
    opts: WebhookOptions,
    chan: Sender<TelegramResult<Update>>,
) -> TelegramResult<()> {
    let addr = SocketAddr::from((opts.ip, opts.port));

    let payload = HandlingPayload::new(&opts, chan.clone());
    let make_svc = make_service_fn(move |_conn| {
        let inner_payload = payload.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_req(inner_payload.clone(), req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        let send_res = chan
            .send(Err(TelegramError::Unknown(e.to_string()).into()))
            .await;
        if send_res.is_err() {
            return Err(TelegramError::WebhookError.into());
        }
    }
    Ok(())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

/// Represents the options to set for the webhook handling
#[derive(Clone, Debug)]
pub struct WebhookOptions {
    pub url: Option<Uri>,
    pub path: String,
    pub port: u16,
    pub ip: IpAddr,
}

impl WebhookOptions {
    /// Creates a new `WebhookOptions` with default values
    ///
    /// By default it will listen on 127.0.0.1:8006 and the path being the root
    pub fn new() -> Self {
        Self {
            url: None,
            path: "/".to_owned(),
            port: 8006,
            ip: [127, 0, 0, 1].into(),
        }
    }

    /// Sets the path of the webhook
    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_owned();
        self
    }

    /// Sets the port the webhook will be listening on
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    /// Sets the IP the webhook will be listening on
    pub fn set_ip<T: Into<IpAddr>>(&mut self, ip: T) -> &mut Self {
        self.ip = ip.into();
        self
    }

    /// Sets the url of the webhook
    pub fn set_url(&mut self, url: &str) -> TelegramResult<&mut Self> {
        self.url = Some(url.parse()?);
        Ok(self)
    }

    fn get_path(&self) -> &str {
        self.url
            .as_ref()
            .map_or_else(|| self.path.as_str(), |url| url.path())
    }
}

impl Default for WebhookOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
struct HandlingPayload {
    path: String,
    chan: Sender<TelegramResult<Update>>,
}

impl HandlingPayload {
    fn new(opts: &WebhookOptions, sender: Sender<TelegramResult<Update>>) -> Self {
        Self {
            path: opts.get_path().to_owned(),
            chan: sender,
        }
    }
}
