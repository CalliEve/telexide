use std::sync::atomic::{AtomicUsize, Ordering};
use telexide::{
    client::{ClientBuilder, Context},
    framework::CommandResult,
    macros::{command, create_framework, prepare_listener},
    model::{
        Chat,
        Message,
        MessageContent,
        MessageEntity,
        PrivateChat,
        TextBlock,
        Update,
        UpdateContent,
    },
    Result,
};

static macro_b: AtomicUsize = AtomicUsize::new(0);

#[prepare_listener]
async fn testing_macro(_c: Context, u: Update) {
    macro_b.fetch_add(u.update_id as usize, Ordering::Acquire);
}

#[tokio::test]
async fn test_using_macro_to_prepare() -> Result<()> {
    let mut c = ClientBuilder::new().set_token("test").build();

    c.subscribe_handler_func(testing_macro);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    });

    tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(macro_b.load(Ordering::Relaxed), 10);
    Ok(())
}

static command_b: AtomicUsize = AtomicUsize::new(0);

#[command(description = "testing")]
async fn testing_command(_c: Context, m: Message) -> CommandResult {
    println!("{}", m.message_id);
    command_b.fetch_add(m.message_id as usize, Ordering::Acquire);
    Ok(())
}

#[tokio::test]
async fn test_using_command() -> Result<()> {
    let mut c = ClientBuilder::new()
        .set_token("test")
        .set_framework(create_framework!("test_bot", testing_command))
        .build();

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
            via_bot: None,
            edit_date: None,
            author_signature: None,
            connected_website: None,
            passport_data: None,
            reply_markup: None,
            content: MessageContent::Unknown,
        }),
    });

    tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(command_b.load(Ordering::Relaxed), 0);

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
            via_bot: None,
            edit_date: None,
            author_signature: None,
            connected_website: None,
            passport_data: None,
            reply_markup: None,
            content: MessageContent::Text {
                content: "/testing_command".to_owned(),
                entities: vec![MessageEntity::BotCommand(TextBlock {
                    offset: 0,
                    length: 16,
                })],
            },
        }),
    });

    tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(command_b.load(Ordering::Relaxed), 30);
    Ok(())
}
