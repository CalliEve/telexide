use super::utils::unix_date_formatting;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::User;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description, for groups, supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link, for groups, supergroups and channel chats.
    pub invite_link: Option<String>,
    /// Pinned message, for groups, supergroups and channels. Returned only in getChat.
    pub pinned_message: Option<Box<super::message::RawMessage>>,
    /// Default chat member permissions, for groups and supergroups. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
    /// For supergroups, the minimum allowed delay between consecutive messages sent by each
    /// unpriviledged user. Returned only in getChat.
    pub slow_mode_delay: Option<usize>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ChatType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    SuperGroup,
    #[serde(rename = "channel")]
    Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PrivateChat {
    pub id: i64,
    /// Username if available
    pub username: Option<String>,
    /// First name of the other party
    pub first_name: Option<String>,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroupChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
    /// Default chat member permissions. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SuperGroupChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Username if available
    pub username: Option<String>,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
    /// Default chat member permissions. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
    /// The minimum allowed delay between consecutive messages sent by each unpriviledged user.
    /// Returned only in getChat.
    pub slow_mode_delay: Option<usize>,
    /// Name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChannelChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Username if available
    pub username: Option<String>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link
    pub invite_link: Option<String>,
    /// Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
}

/// This object represents a chat. It can be a private, group, supergroup or channel chat
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatPermissions {
    #[serde(default)]
    pub can_send_messages: bool,
    #[serde(default)]
    pub can_send_media_messages: bool,
    #[serde(default)]
    pub can_send_polls: bool,
    #[serde(default)]
    pub can_send_other_messages: bool,
    #[serde(default)]
    pub can_add_web_page_previews: bool,
    #[serde(default)]
    pub can_change_info: bool,
    #[serde(default)]
    pub can_invite_users: bool,
    #[serde(default)]
    pub can_pin_messages: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    /// This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    /// This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}

impl Chat {
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
            }),
            ChatType::Private => Chat::Private(PrivateChat {
                id: raw.id,
                first_name: raw.first_name,
                last_name: raw.last_name,
                username: raw.username,
                photo: raw.photo,
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
                title: None,
                description: None,
                pinned_message: None,
                invite_link: None,
                permissions: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                slow_mode_delay: None,
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
                permissions: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                slow_mode_delay: None,
                first_name: None,
                last_name: None,
            },
        }
    }
}

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
    Kicked(KickedMemberStatus)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreatorMemberStatus {
    /// Information about the user
    pub user: User,
    /// Custom title for this user
    pub custom_title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdministratorMemberStatus {
    /// Information about the user
    pub user: User,
    /// Custom title for this user
    pub custom_title: Option<String>,
    /// True, if the bot is allowed to edit administrator privileges of that use
    #[serde(default)]
    pub can_be_edited: bool,
    /// True, if the administrator can post in the channel; channels only
    #[serde(default)]
    pub can_post_messages: bool,
    /// True, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(default)]
    pub can_edit_messages: bool,
    /// True, if the administrator can delete messages of other users
    #[serde(default)]
    pub can_delete_messages: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    #[serde(default)]
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of his own privileges
    /// or demote administrators that he has promoted, directly or indirectly
    /// (promoted by administrators that were appointed by the user)
    #[serde(default)]
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    #[serde(default)]
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(default)]
    pub can_invite_users: bool,
    ///  True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(default)]
    pub can_pin_messages: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MemberMemberStatus {
    /// Information about the user
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RestrictedMemberStatus {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
    #[serde(default)]
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(default)]
    pub can_invite_users: bool,
    ///  True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(default)]
    pub can_pin_messages: bool,
    /// True, if the user is a member of the chat at the moment of the request
    #[serde(default)]
    pub is_member: bool,
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(default)]
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
    #[serde(default)]
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls
    #[serde(default)]
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots
    #[serde(default)]
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    #[serde(default)]
    pub can_add_web_page_previews: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LeftMemberStatus {
    /// Information about the user
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KickedMemberStatus {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
}

impl ChatMember {
    fn get_user(&self) -> &User {
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
