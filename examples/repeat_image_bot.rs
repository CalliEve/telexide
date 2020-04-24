#![allow(where_clauses_object_safety)]
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use telexide::{api::types::{SendMessage, UpdateType, SendPhoto}, model::{UpdateContent, MessageContent}, prelude::*};
use typemap::Key as TypeMapKey;
use parking_lot::RwLock;

struct HashMapKey;
impl TypeMapKey for HashMapKey {
    type Value = Arc<RwLock<HashMap<i64, i64>>>;
}

#[command(description = "repeat the next image")]
async fn repeat(context: Context, message: Message) {
    if message.from.is_none() {
        return;
    }

    let res = context
        .api
        .send_message(
            SendMessage::new(message.chat.get_id(), "please send the image I will repeat")
        )
        .await;
    if res.is_err() {
        println!("got an error when sending the asking message: {}", res.err().unwrap());
        return;
    }

    let mut guard = context.data.write();
    let mut map = guard.get_mut::<HashMapKey>().expect("no hashmap").clone();
    map.write().insert(message.chat.get_id(), message.from.as_ref().expect("no author").id);
}

#[prepare_listener]
async fn handle_next(context: Context, update: Update) {
    let message = match update.content {
        UpdateContent::Message(ref m) => m,
        _ => return,
    };

    if message.from.is_none() {
        return;
    }

    let image = match message.content {
        MessageContent::Photo {ref content, .. } => content.first(),
        _ => return,
    };

    if image.is_none() {
        return;
    }

    {
        let mut guard = context.data.write();
        let mut maplock = guard.get_mut::<HashMapKey>().expect("no hashmap").clone();
        let mut map = maplock.write();

        let key = match map.get(&message.chat.get_id()) {
            Some(u) if *u != message.from.as_ref().expect("no author").id => return,
            Some(u) => *u,
            None => return,
        };

        map.remove(&key);
    }

    let res = context
        .api
        .send_photo(
            SendPhoto::from_photo_size(message.chat.get_id(), &image.expect("no image"))
        )
        .await;
    if res.is_err() {
        println!("got an error when sending the asking message: {}", res.err().unwrap());
        return;
    }
}

#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    let client = ClientBuilder::new()
        .set_token(token)
        .set_framework(create_framework!(&bot_name, repeat))
        .add_allowed_updates(UpdateType::Message)
        .add_handler_func(handle_next)
        .build();

    {
        let mut data = client.data.write();
        data.insert::<HashMapKey>(Arc::new(RwLock::new(HashMap::new())));
    }

    client.start().await
}
