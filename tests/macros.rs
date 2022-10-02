use std::sync::atomic::{AtomicUsize, Ordering};
use telexide::{
    client::{ClientBuilder, Context},
    framework::CommandResult,
    macros::{command, create_framework, prepare_listener},
    model::{
        Chat, Message, MessageContent, MessageEntity, PrivateChat, TextBlock, Update, UpdateContent,
    },
    Result,
};

static MACRO_B: AtomicUsize = AtomicUsize::new(0);

#[prepare_listener]
async fn testing_macro(_c: Context, u: Update) {
    MACRO_B.fetch_add(u.update_id as usize, Ordering::Acquire);
}

#[tokio::test]
async fn test_using_macro_to_prepare() -> Result<()> {
    let mut c = ClientBuilder::new().set_token("test").build();

    c.subscribe_handler_func(testing_macro);

    c.fire_handlers(Update {
        update_id: 10,
        content: UpdateContent::Unknown,
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(MACRO_B.load(Ordering::Relaxed), 10);
    Ok(())
}

static COMMAND_B: AtomicUsize = AtomicUsize::new(0);

#[command(description = "testing")]
async fn testing_command(_c: Context, m: Message) -> CommandResult {
    println!("{}", m.message_id);
    COMMAND_B.fetch_add(m.message_id as usize, Ordering::Acquire);
    Ok(())
}

#[tokio::test]
async fn test_using_command() -> Result<()> {
    let c = ClientBuilder::new()
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
                bio: None,
                last_name: None,
                photo: None,
                has_private_forwards: None,
                has_restricted_voice_and_video_messages: None,
                message_auto_delete_time: None,
            }),
            sender_chat: None,
            forward_data: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            author_signature: None,
            connected_website: None,
            passport_data: None,
            reply_markup: None,
            has_protected_content: false,
            content: MessageContent::Unknown,
        }),
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(COMMAND_B.load(Ordering::Relaxed), 0);

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
                bio: None,
                last_name: None,
                photo: None,
                has_private_forwards: None,
                has_restricted_voice_and_video_messages: None,
                message_auto_delete_time: None,
            }),
            sender_chat: None,
            forward_data: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            author_signature: None,
            connected_website: None,
            passport_data: None,
            reply_markup: None,
            has_protected_content: false,
            content: MessageContent::Text {
                content: "/testing_command".to_owned(),
                entities: vec![MessageEntity::BotCommand(TextBlock {
                    offset: 0,
                    length: 16,
                })],
            },
        }),
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    assert_eq!(COMMAND_B.load(Ordering::Relaxed), 30);
    Ok(())
}
