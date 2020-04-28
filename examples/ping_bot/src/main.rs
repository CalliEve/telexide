use std::env;
use telexide::{api::types::SendMessage, prelude::*};

#[command(description = "just a ping-pong command")]
async fn ping(context: Context, message: Message) -> CommandResult {
    context
        .api
        .send_message(SendMessage::new(message.chat.get_id(), "pong"))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    ClientBuilder::new()
        .set_token(&token)
        .set_framework(create_framework!(&bot_name, ping))
        .build()
        .start()
        .await
}
