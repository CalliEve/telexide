use std::sync::atomic::{AtomicUsize, Ordering};
use telegram_botoxide::{
    client::{Client, Context},
    macros::{command, prepare_listener},
    model::{Chat, Message, MessageContent, PrivateChat, Update, UpdateContent},
    options::OptionsBuilder,
    Result,
};

static macro_b: AtomicUsize = AtomicUsize::new(0);

#[prepare_listener]
async fn testing_macro(_c: Context, u: Update) {
    macro_b.fetch_add(u.update_id as usize, Ordering::Acquire);
}

#[tokio::test]
async fn test_using_macro_to_prepare() -> Result<()> {
    let mut c = Client::new(&OptionsBuilder::new().set_token("test".to_owned()).build()?);

    c.subscribe_handler(testing_macro);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    })
    .await;

    assert_eq!(macro_b.load(Ordering::Relaxed), 10);
    Ok(())
}

static command_b: AtomicUsize = AtomicUsize::new(0);

#[prepare_listener]
async fn testing_command(_c: Context, m: Message) {
    command_b.fetch_add(m.message_id as usize, Ordering::Acquire);
}

#[tokio::test]
async fn test_using_command() -> Result<()> {
    let mut c = Client::new(&OptionsBuilder::new().set_token("test".to_owned()).build()?);

    c.subscribe_handler(testing_macro);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Message(Message {
            message_id: 30,
            from: None,
            date: chrono::offset::Utc::now(),
            chat: Chat::Private(PrivateChat {
                id: 40,
                username: None,
                first_name: None,
                last_name: None,
                photo: None,
            }),
            forward_data: None,
            reply_to_message: None,
            edit_date: None,
            author_signature: None,
            connected_website: None,
            passport_data: None,
            reply_markup: None,
            content: MessageContent::Unknown,
        }),
    })
    .await;

    assert_eq!(command_b.load(Ordering::Relaxed), 30);
    Ok(())
}
