use crate::model::utils::IntegerOrString;
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`create_forum_topic`]
///
/// [`create_forum_topic`]:
/// ../../api/trait.API.html#method.create_forum_topic
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Topic name, 1-128 characters
    pub name: String,
    /// Color of the topic icon in RGB format.
    /// Currently, must be one of 0x6FB9F0, 0xFFD67E, 0xCB86DB, 0x8EEE98, 0xFF93B2, or 0xFB6F5F.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    /// Unique identifier of the custom emoji shown as the topic icon.
    /// Use [`get_forum_topic_icon_stickers`] to get all allowed custom emoji identifiers.
    ///
    /// [`get_forum_topic_icon_stickers`]: ../../api/trait.API.html#method.get_forum_topic_icon_stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// struct for holding data needed to call
/// [`edit_forum_topic`]
///
/// [`edit_forum_topic`]:
/// ../../api/trait.API.html#method.edit_forum_topic
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EditForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
    /// Topic name, 1-128 characters
    pub name: String,
    /// Unique identifier of the custom emoji shown as the topic icon.
    /// Use [`get_forum_topic_icon_stickers`] to get all allowed custom emoji identifiers.
    ///
    /// [`get_forum_topic_icon_stickers`]: ../../api/trait.API.html#method.get_forum_topic_icon_stickers
    pub icon_custom_emoji_id: String,
}

/// struct for holding data needed to call
/// [`close_forum_topic`]
///
/// [`close_forum_topic`]:
/// ../../api/trait.API.html#method.close_forum_topic
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CloseForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

/// struct for holding data needed to call
/// [`reopen_forum_topic`]
///
/// [`reopen_forum_topic`]:
/// ../../api/trait.API.html#method.reopen_forum_topic
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReopenForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

/// struct for holding data needed to call
/// [`delete_forum_topic`]
///
/// [`delete_forum_topic`]:
/// ../../api/trait.API.html#method.delete_forum_topic
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

/// struct for holding data needed to call
/// [`unpin_all_forum_topic_messages`]
///
/// [`cunpin_all_forum_topic_messages`]:
/// ../../api/trait.API.html#method.unpin_all_forum_topic_messages
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UnpinAllForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup
    pub chat_id: IntegerOrString,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
