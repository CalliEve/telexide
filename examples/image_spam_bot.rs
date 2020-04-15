#![allow(where_clauses_object_safety)]
use std::env;
use telexide::{api::types::{SendMediaGroup, UpdateType, InputMedia, InputMediaPhoto, InputFile}, prelude::*};

#[command(description = "just a ping-pong command", name = "spaceimage")]
async fn space_image(context: Context, message: Message) {
    let mut media = Vec::new();
    loop {
        media.push(InputMedia::Photo(
            InputMediaPhoto {
                media: InputFile::from_path("./examples/silver_coin_galaxy.jpg").expect("error while getting file"),
                caption: Some("Take a look at this awesome galaxy!".to_owned()),
                parse_mode: None
            }
        ));
        if media.len() > 3 {
            break
        }
    }

    let mut data = SendMediaGroup::new(
        message.chat.get_id(),
        media
    );

    let res = context
        .api
        .send_media_group(
            data
        )
        .await;
    if res.is_err() {
        println!("got an error when sending the space image message: {}", res.err().unwrap())
    }
}

#[tokio::main]
async fn main() -> telexide::Result<()> {
    let token = env::var("BOT_TOKEN").expect("no token environment variable set");
    let bot_name = env::var("BOT_NAME").expect("no bot name env variable set");

    ClientBuilder::new()
        .set_token(token)
        .set_framework(create_framework!(&bot_name, space_image))
        .add_allowed_updates(UpdateType::Message)
        .build()
        .start()
        .await
}
