use chrono::{DateTime, Utc};
use crate::model::{ChatPermissions, Chat, utils::unix_date_formatting};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KickChatMember {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UnbanChatMember {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New user permissions
    pub permissions: ChatPermissions,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// if the administrator can add new administrators with a subset of his own privileges
    /// or demote administrators that he has promoted, directly or indirectly
    /// (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}

impl PromoteChatMember {
    pub fn new(chat_id: i64, user_id: i64) -> Self {
        Self {
            chat_id,
            user_id,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// New default chat permissions
    pub permissions: ChatPermissions
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    // TODO: support file uploading
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// New chat title, 1-255 characters
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PinChatMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LeaveChat {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChat {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetChatMember {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat
    pub chat_id: i64,
}

macro_rules! impl_from_chat {
    ($name:ident) => {
        impl From<Chat> for $name {
            fn from(chat: Chat) -> Self {
                Self {
                    chat_id: chat.get_id()
                }
            }
        }
    }
}

impl_from_chat!(ExportChatInviteLink);
impl_from_chat!(DeleteChatPhoto);
impl_from_chat!(UnpinChatMessage);
impl_from_chat!(LeaveChat);
impl_from_chat!(GetChat);
impl_from_chat!(GetChatAdministrators);
impl_from_chat!(GetChatMembersCount);
impl_from_chat!(DeleteChatStickerSet);
