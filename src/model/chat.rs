use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    raw::{ChatType, RawChat},
    utils::unix_date_formatting,
    User,
};

/// A private chat object, also known as a DM, between the bot and an user
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PrivateChat {
    /// Unique identifier for this chat
    pub id: i64,
    /// Username if available
    pub username: Option<String>,
    /// First name of the other party
    pub first_name: Option<String>,
    /// Bio of the other party in a private chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub bio: Option<String>,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
}

/// A Group chat object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroupChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub pinned_message: Option<Box<super::Message>>,
    /// Default chat member permissions. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub permissions: Option<super::ChatPermissions>,
}

/// A supergroup object (a group with more than 200 members)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SuperGroupChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Username if available
    pub username: Option<String>,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub pinned_message: Option<Box<super::Message>>,
    /// Default chat member permissions. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub permissions: Option<super::ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each
    /// unprivileged user. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub slow_mode_delay: Option<usize>,
    /// Name of group sticker set. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub can_set_sticker_set: Option<bool>,
    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa; for supergroups and channel
    /// chats. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub linked_chat_id: Option<i64>,
    /// For supergroups, the location to which the supergroup is connected.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub location: Option<ChatLocation>,
}

/// A Channel object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChannelChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Username if available
    pub username: Option<String>,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub pinned_message: Option<Box<super::Message>>,
    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa; for supergroups and channel
    /// chats. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub linked_chat_id: Option<i64>,
}

/// This object represents a chat. It can be a private, group, supergroup or
/// channel chat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Chat {
    #[serde(rename = "private")]
    Private(PrivateChat),
    #[serde(rename = "group")]
    Group(GroupChat),
    #[serde(rename = "supergroup")]
    SuperGroup(SuperGroupChat),
    #[serde(rename = "channel")]
    Channel(ChannelChat),
}

/// Represents a location to which a chat is connected.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live
    /// location.
    pub location: super::Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}

/// Describes actions that a non-administrator user is allowed to take in a
/// chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatPermissions {
    /// True, if the user is allowed to send text messages, contacts, locations
    /// and venues.
    #[serde(default)]
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios, documents, photos, videos,
    /// video notes and voice notes, implies can_send_messages to be true.
    #[serde(default)]
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls, implies can_send_messages to
    /// be true.
    #[serde(default)]
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use
    /// inline bots, implies can_send_media_messages to be true.
    #[serde(default)]
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages,
    /// implies can_send_media_messages to be true.
    #[serde(default)]
    pub can_add_web_page_previews: bool,
    /// True, if the user is allowed to change the chat title, photo and other
    /// settings. Ignored in public supergroups.
    #[serde(default)]
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat.
    #[serde(default)]
    pub can_invite_users: bool,
    /// True, if the user is allowed to pin messages. Ignored in public
    /// supergroups.
    #[serde(default)]
    pub can_pin_messages: bool,
}

/// This object represents a chat photo.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    /// This file_id can be used only for photo download and only for as long as
    /// the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed
    /// to be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download and only for as long as
    /// the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to
    /// be the same over time and for different bots. Can't be used to
    /// download or reuse the file.
    pub big_file_unique_id: String,
}

impl Chat {
    /// Gets the id of the chat
    pub fn get_id(&self) -> i64 {
        match self {
            Chat::Private(c) => c.id,
            Chat::Channel(c) => c.id,
            Chat::Group(c) => c.id,
            Chat::SuperGroup(c) => c.id,
        }
    }
}

impl From<RawChat> for Chat {
    fn from(raw: RawChat) -> Chat {
        match raw.chat_type {
            ChatType::Channel => Chat::Channel(ChannelChat {
                id: raw.id,
                title: raw.title.unwrap_or_default(),
                username: raw.username,
                photo: raw.photo,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                linked_chat_id: raw.linked_chat_id,
            }),
            ChatType::Private => Chat::Private(PrivateChat {
                id: raw.id,
                first_name: raw.first_name,
                last_name: raw.last_name,
                username: raw.username,
                photo: raw.photo,
                bio: raw.bio,
            }),
            ChatType::Group => Chat::Group(GroupChat {
                id: raw.id,
                title: raw.title.unwrap_or_default(),
                photo: raw.photo,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                permissions: raw.permissions,
            }),
            ChatType::SuperGroup => Chat::SuperGroup(SuperGroupChat {
                id: raw.id,
                title: raw.title.unwrap_or_default(),
                username: raw.username,
                photo: raw.photo,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                permissions: raw.permissions,
                sticker_set_name: raw.sticker_set_name,
                can_set_sticker_set: raw.can_set_sticker_set,
                slow_mode_delay: raw.slow_mode_delay,
                linked_chat_id: raw.linked_chat_id,
                location: raw.location,
            }),
        }
    }
}

impl From<Chat> for RawChat {
    fn from(chat: Chat) -> RawChat {
        match chat {
            Chat::Private(c) => RawChat {
                chat_type: ChatType::Private,
                first_name: c.first_name,
                last_name: c.last_name,
                id: c.id,
                username: c.username,
                photo: c.photo,
                bio: c.bio,
                title: None,
                description: None,
                pinned_message: None,
                invite_link: None,
                permissions: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                slow_mode_delay: None,
                linked_chat_id: None,
                location: None,
            },
            Chat::Group(c) => RawChat {
                chat_type: ChatType::Group,
                id: c.id,
                title: Some(c.title),
                photo: c.photo,
                description: c.description,
                pinned_message: c.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: c.invite_link,
                permissions: c.permissions,
                username: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                slow_mode_delay: None,
                first_name: None,
                last_name: None,
                bio: None,
                linked_chat_id: None,
                location: None,
            },
            Chat::SuperGroup(c) => RawChat {
                chat_type: ChatType::SuperGroup,
                id: c.id,
                title: Some(c.title),
                username: c.username,
                photo: c.photo,
                description: c.description,
                pinned_message: c.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: c.invite_link,
                permissions: c.permissions,
                sticker_set_name: c.sticker_set_name,
                can_set_sticker_set: c.can_set_sticker_set,
                slow_mode_delay: c.slow_mode_delay,
                linked_chat_id: c.linked_chat_id,
                location: c.location,
                bio: None,
                first_name: None,
                last_name: None,
            },
            Chat::Channel(c) => RawChat {
                chat_type: ChatType::Channel,
                id: c.id,
                title: Some(c.title),
                username: c.username,
                photo: c.photo,
                description: c.description,
                pinned_message: c.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: c.invite_link,
                linked_chat_id: c.linked_chat_id,
                permissions: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                slow_mode_delay: None,
                first_name: None,
                last_name: None,
                bio: None,
                location: None,
            },
        }
    }
}

/// This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Creator(CreatorMemberStatus),
    #[serde(rename = "administrator")]
    Administrator(AdministratorMemberStatus),
    #[serde(rename = "member")]
    Member(MemberMemberStatus),
    #[serde(rename = "restricted")]
    Restricted(RestrictedMemberStatus),
    #[serde(rename = "left")]
    Left(LeftMemberStatus),
    #[serde(rename = "kicked")]
    Kicked(KickedMemberStatus),
}

/// Represents a [`ChatMember`] who is the creator of the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreatorMemberStatus {
    /// Information about the user
    pub user: User,
    /// Custom title for this user
    pub custom_title: Option<String>,
    /// Owner and administrators only. True, if the user's presence in the chat
    /// is hidden
    #[serde(default)]
    pub is_anonymous: bool,
}

/// Represents a [`ChatMember`] who is an Admin of the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdministratorMemberStatus {
    /// Information about the user
    pub user: User,
    /// Custom title for this user
    pub custom_title: Option<String>,
    /// Owner and administrators only. True, if the user's presence in the chat
    /// is hidden
    #[serde(default)]
    pub is_anonymous: bool,
    /// True, if the bot is allowed to edit administrator privileges of that
    /// user
    #[serde(default)]
    pub can_be_edited: bool,
    /// True, if the administrator can access the chat event log, chat
    /// statistics, message statistics in channels, see channel members, see
    /// anonymous administrators in supergroups and ignore slow mode.
    /// Implied by any other administrator privilege
    #[serde(default)]
    pub can_manage_chat: bool,
    /// True, if the administrator can post in the channel; channels only
    #[serde(default)]
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls
    #[serde(default)]
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use
    /// inline bots
    #[serde(default)]
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    #[serde(default)]
    pub can_add_web_page_previews: bool,
    /// True, if the administrator can manage voice chats
    #[serde(default)]
    pub can_manage_voice_chats: bool,
}

/// Represents a [`ChatMember`] who is a normal member of the [`Chat`] without
/// any special powers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MemberMemberStatus {
    /// Information about the user
    pub user: User,
}

/// Represents a restricted [`ChatMember`] of a [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RestrictedMemberStatus {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
    #[serde(default)]
    /// True, if the user is allowed to change the chat title, photo and other
    /// settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(default)]
    pub can_invite_users: bool,
    ///  True, if the user is allowed to pin messages; groups and supergroups
    /// only
    #[serde(default)]
    pub can_pin_messages: bool,
    /// True, if the user is a member of the chat at the moment of the request
    #[serde(default)]
    pub is_member: bool,
    /// True, if the user is allowed to send text messages, contacts, locations
    /// and venues
    #[serde(default)]
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios, documents, photos, videos,
    /// video notes and voice notes
    #[serde(default)]
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls
    #[serde(default)]
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use
    /// inline bots
    #[serde(default)]
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    #[serde(default)]
    pub can_add_web_page_previews: bool,
}

/// Represents a [`ChatMember`] who left the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LeftMemberStatus {
    /// Information about the user
    pub user: User,
}

/// Represents a [`ChatMember`] who has been kicked from the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KickedMemberStatus {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
}

impl ChatMember {
    /// Retrieves the underlying [`User`] of the [`ChatMember`].
    pub fn get_user(&self) -> &User {
        match self {
            ChatMember::Administrator(m) => &m.user,
            ChatMember::Creator(m) => &m.user,
            ChatMember::Kicked(m) => &m.user,
            ChatMember::Left(m) => &m.user,
            ChatMember::Member(m) => &m.user,
            ChatMember::Restricted(m) => &m.user,
        }
    }
}

/// Represents an invite link for a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// If the link is primary
    pub is_primary: bool,
    /// If the link is revoked
    pub is_revoked: bool,
    /// When the link will expire or has been expired
    #[serde(with = "unix_date_formatting::optional")]
    pub expire_date: Option<DateTime<Utc>>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    #[serde(default)]
    pub member_limit: Option<i32>,
}

/// Represents changes in the status of a chat member.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done
    #[serde(with = "unix_date_formatting")]
    pub date: DateTime<Utc>,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for
    /// joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}
