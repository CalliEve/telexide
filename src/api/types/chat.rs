use super::InputFile;
use crate::model::{
    utils::{unix_date_formatting, IntegerOrString},
    Chat, ChatPermissions,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`ban_chat_member`]
///
/// [`ban_chat_member`]:
/// ../../api/trait.API.html#method.ban_chat_member
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BanChatMember {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from
    /// the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass True to delete all messages from the chat for the user that is
    /// being removed. If False, the user will be able to see messages in
    /// the group that were sent before the user was removed. Always True
    /// for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

/// struct for holding data needed to call
/// [`unban_chat_member`]
///
/// [`unban_chat_member`]:
/// ../../api/trait.API.html#method.unban_chat_member
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnbanChatMember {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

/// struct for holding data needed to call
/// [`restrict_chat_member`]
///
/// [`restrict_chat_member`]:
/// ../../api/trait.API.html#method.restrict_chat_member
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New user permissions
    pub permissions: ChatPermissions,
    /// Date when the user will be unbanned, unix time.
    /// If user is banned for more than 366 days or less than 30 seconds from
    /// the current time they are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
}

/// struct for holding data needed to call
/// [`promote_chat_member`]
///
/// [`promote_chat_member`]:
/// ../../api/trait.API.html#method.promote_chat_member
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// If the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// If the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// If the administrator can edit messages of other users and can pin
    /// messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// If the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// If the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// If the administrator can add new administrators with a subset of his own
    /// privileges or demote administrators that he has promoted, directly
    /// or indirectly (promoted by administrators that were appointed by
    /// him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// If the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// If the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// If the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// If the administrator can manage video chats, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// If the administrator can access the chat event log, chat statistics,
    /// message statistics in channels, see channel members, see anonymous
    /// administrators in supergroups and ignore slow mode. Implied by any
    /// other administrator privilege.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// If the user is allowed to create, rename, close, and reopen forum
    /// topics, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

/// struct for holding data needed to call
/// [`set_chat_administrator_custom_title`]
///
/// [`set_chat_administrator_custom_title`]:
/// ../../api/trait.API.html#method.set_chat_administrator_custom_title
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not
    /// allowed
    pub custom_title: String,
}

/// struct for holding data needed to call [`ban_chat_sender_chat`]
///
/// [`ban_chat_sender_chat`]:
/// ../../api/trait.API.html#method.ban_chat_sender_chat
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

/// struct for holding data needed to call [`unban_chat_sender_chat`]
///
/// [`unban_chat_sender_chat`]:
/// ../../api/trait.API.html#method.unban_chat_sender_chat
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

/// struct for holding data needed to call [`set_chat_permissions`]
///
/// [`set_chat_permissions`]:
/// ../../api/trait.API.html#method.set_chat_permissions
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// New default chat permissions
    pub permissions: ChatPermissions,
}

/// struct for holding data needed to call [`export_chat_invite_link`]
///
/// [`export_chat_invite_link`]:
/// ../../api/trait.API.html#method.export_chat_invite_link
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`set_chat_photo`]
///
/// [`set_chat_photo`]:
/// ../../api/trait.API.html#method.set_chat_photo
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// New chat photo
    pub photo: InputFile,
}

/// struct for holding data needed to call
/// [`delete_chat_photo`]
///
/// [`delete_chat_photo`]:
/// ../../api/trait.API.html#method.delete_chat_photo
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`set_chat_title`]
///
/// [`set_chat_title`]:
/// ../../api/trait.API.html#method.set_chat_title
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// New chat title, 1-255 characters
    pub title: String,
}

/// struct for holding data needed to call
/// [`set_chat_description`]
///
/// [`set_chat_description`]:
/// ../../api/trait.API.html#method.set_chat_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// struct for holding data needed to call
/// [`pin_chat_message`]
///
/// [`pin_chat_message`]:
/// ../../api/trait.API.html#method.pin_chat_message
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PinChatMessage {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

/// struct for holding data needed to call
/// [`unpin_chat_message`]
///
/// [`unpin_chat_message`]:
/// ../../api/trait.API.html#method.unpin_chat_message
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Identifier of a message to unpin. If not specified, the most recent
    /// pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

/// struct for holding data needed to call
/// [`unpin_all_chat_messages`]
///
/// [`unpin_all_chat_messages`]:
/// ../../api/trait.API.html#method.unpin_all_chat_message
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`leave_chat`]
///
/// [`leave_chat`]:
/// ../../api/trait.API.html#method.leave_chat
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LeaveChat {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`get_chat`]
///
/// [`get_chat`]:
/// ../../api/trait.API.html#method.get_chat
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChat {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`get_chat_administrator`]
///
/// [`get_chat_administrator`]:
/// ../../api/trait.API.html#method.get_chat_administrator
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`get_chat_member_count`]
///
/// [`get_chat_member_count`]:
/// ../../api/trait.API.html#method.get_chat_member_count
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call
/// [`get_chat_member`]
///
/// [`get_chat_member`]:
/// ../../api/trait.API.html#method.get_chat_member
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetChatMember {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
}

/// struct for holding data needed to call
/// [`set_chat_sticker_set`]
///
/// [`set_chat_sticker_set`]:
/// ../../api/trait.API.html#method.get_chat_sticker_set
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

/// struct for holding data needed to call
/// [`delete_chat_sticker_set`]
///
/// [`delete_chat_sticker_set`]:
/// ../../api/trait.API.html#method.delete_chat_sticker_set
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
}

/// struct for holding data needed to call [`create_chat_invite_link`]
///
/// [`create_chat_invite_link`]:
/// ../../api/trait.API.html#method.create_chat_invite_link
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Invite link name; 0-32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// True, if users joining the chat via the link need to be approved by chat
    /// administrators. If True, member_limit can't be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

/// struct for holding data needed to call [`edit_chat_invite_link`]
///
/// [`edit_chat_invite_link`]:
/// ../../api/trait.API.html#method.edit_chat_invite_link
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EditChatInviteLink {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// True, if users joining the chat via the link need to be approved by chat
    /// administrators. If True, member_limit can't be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

/// struct for holding data needed to call [`revoke_chat_invite_link`]
///
/// [`revoke_chat_invite_link`]:
/// ../../api/trait.API.html#method.revoke_chat_invite_link
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RevokeChatInviteLink {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// The invite link to revoke
    pub invite_link: String,
}

/// struct for holding data needed to call [`approve_chat_join_request`]
///
/// [`approve_chat_join_request`]:
/// ../../api/trait.API.html#method.approve_chat_join_request
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
}

/// struct for holding data needed to call [`decline_chat_join_request`]
///
/// [`decline_chat_join_request`]:
/// ../../api/trait.API.html#method.decline_chat_join_request
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat
    pub chat_id: IntegerOrString,
    /// Unique identifier of the target user
    pub user_id: i64,
}

macro_rules! impl_from_chat {
    ($name:ident) => {
        impl From<Chat> for $name {
            fn from(chat: Chat) -> Self {
                Self {
                    chat_id: chat.get_id().into(),
                }
            }
        }
    };
}

impl_from_chat!(ExportChatInviteLink);
impl_from_chat!(DeleteChatPhoto);
impl_from_chat!(UnpinAllChatMessages);
impl_from_chat!(LeaveChat);
impl_from_chat!(GetChat);
impl_from_chat!(GetChatAdministrators);
impl_from_chat!(GetChatMemberCount);
impl_from_chat!(DeleteChatStickerSet);
