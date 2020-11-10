use hyper;
use std::sync::atomic::{AtomicUsize, Ordering};
use telexide::{
    client::{Webhook, WebhookOptions},
    model::{Update, UpdateContent},
    Result,
};
use tokio::sync::mpsc::Receiver;

static ATOMIC: AtomicUsize = AtomicUsize::new(0);

async fn webhook_receiver_handler(mut receiver: Receiver<Result<Update>>) {
    while let Some(u_res) = receiver.recv().await {
        if let Ok(u) = u_res {
            ATOMIC.fetch_add(u.update_id as usize, Ordering::Acquire);
        } else {
            panic!("returned error from receiver")
        }
    }
}

#[tokio::test]
async fn webhook_gets_called() -> Result<()> {
    let client = hyper::Client::new();

    let mut webhook_opts = WebhookOptions::new();
    webhook_opts.path = "/testing/webhook".to_owned();

    let update_receiver = Webhook::new(&webhook_opts).start();
    tokio::spawn(webhook_receiver_handler(update_receiver));
    tokio::time::delay_for(tokio::time::Duration::from_millis(150)).await;

    let req = hyper::Request::post("http://localhost:8006/testing/webhook")
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .body(hyper::Body::from(serde_json::to_string(&Update {
            update_id: 10,
            content: UpdateContent::Unknown,
        })?))?;
    client.request(req).await?;

    tokio::time::delay_for(tokio::time::Duration::from_millis(150)).await;
    assert_eq!(ATOMIC.load(Ordering::Relaxed), 10);
    Ok(())
}
