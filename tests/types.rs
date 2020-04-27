use telexide::model::{Chat, Message, MessageContent, User};

#[test]
fn decode_user() -> serde_json::Result<()> {
    let t = r#"{
            "id": 456,
            "is_bot": true,
            "first_name": "x",
            "last_name": null
        }"#;

    let u: User = serde_json::from_str(t)?;

    assert_eq!(u.id, 456);
    assert_eq!(u.last_name, None);
    assert_eq!(u.username, None);
    Ok(())
}

#[test]
fn decode_message() -> serde_json::Result<()> {
    let t = r#"{
            "message_id": 16373892,
            "date": 1585772722,
            "chat": {
                "id": 538733,
                "type": "private",
                "first_name": "test"
            },
            "document": {
                "file_id": "test-file",
                "file_unique_id": "testing1"
            },
            "caption": "just testing"
        }"#;

    let m: Message = serde_json::from_str(t)?;

    if let MessageContent::Document {
        caption, ..
    } = m.content.clone()
    {
        assert_eq!(caption, Some("just testing".to_owned()))
    } else {
        panic!("no document")
    }

    if let Chat::Private(c) = m.chat.clone() {
        assert_eq!(c.first_name, Some("test".to_owned()))
    } else {
        panic!("no private chat")
    }

    Ok(())
}
