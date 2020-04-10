#![allow(where_clauses_object_safety)]
use std::env;
use telexide::{api::types::{SendMessage, UpdateType}, prelude::*};

#[command(description = "just a ping-pong command")]
async fn ping(context: Context, message: Message) {
    let res = context
        .api
        .send_message(
            SendMessage::new(message.chat.get_id(), "pong")
        )
        .await;
    if res.is_err() {
        println!("got an error when sending the pong message: {}", res.err().unwrap())
    }
}

#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    ClientBuilder::new()
        .set_token(token)
        .set_framework(create_framework!(&bot_name, ping))
        .add_allowed_updates(UpdateType::Message)
        .build()
        .start()
        .await
}
