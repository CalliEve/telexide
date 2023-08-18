use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{raw::RawChat, utils::unix_date_formatting, User};

/// A private chat object, also known as a DM, between the bot and an user
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// True, if privacy settings of the other party in the private chat allows
    /// to use `tg://user?id=<user_id>` links only in chats with the user.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_private_forwards: bool,
    /// True, if the privacy settings of the other party restrict sending voice
    /// and video note messages in the private chat. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// Last name of the other party
    pub last_name: Option<String>,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all active chat usernames. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub active_usernames: Vec<String>,
    /// Custom emoji identifier of emoji status of the other party in a private
    /// chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub emoji_status_custom_emoji_id: Option<String>,
    /// Expiration date of the emoji status of the other party in a private
    /// chat, if any. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub emoji_status_expiration_date: Option<DateTime<Utc>>,
    /// The time after which all messages sent to the chat will be automatically
    /// deleted; in seconds. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub message_auto_delete_time: Option<usize>,
}

/// A Group chat object
#[derive(Debug, Clone, PartialEq)]
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
    /// True, if non-administrators can only get the list of bots and
    /// administrators in the chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_hidden_members: bool,
    /// True, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_protected_content: bool,
}

/// A supergroup object (a group with more than 200 members)
#[derive(Debug, Clone, PartialEq)]
pub struct SuperGroupChat {
    pub id: i64,
    /// Title
    pub title: String,
    /// Username if available
    pub username: Option<String>,
    /// True, if the supergroup chat is a forum (has [topics] enabled)
    ///
    /// [topics]: https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups
    pub is_forum: bool,
    /// Chat photo. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub photo: Option<ChatPhoto>,
    /// If non-empty, the list of all active chat usernames. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub active_usernames: Vec<String>,
    /// True, if users need to join the supergroup before they can send
    /// messages.Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub join_to_send_messages: bool,
    /// True, if all users directly joining the supergroup need to be approved
    /// by supergroup administrators.Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub join_by_request: bool,
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
    /// True, if aggressive anti-spam checks are enabled in the supergroup. The
    /// field is only available to chat administrators. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_aggressive_anti_spam_enabled: bool,
    /// True, if non-administrators can only get the list of bots and
    /// administrators in the chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_hidden_members: bool,
    /// True, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_protected_content: bool,
    /// Name of group sticker set. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub can_set_sticker_set: bool,
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
#[derive(Debug, Clone, PartialEq)]
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
    /// If non-empty, the list of all active chat usernames. Returned only in
    /// [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub active_usernames: Vec<String>,
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
    /// True, if non-administrators can only get the list of bots and
    /// administrators in the chat. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_hidden_members: bool,
    /// True, if messages from the chat can't be forwarded to other chats.
    /// Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub has_protected_content: bool,
    /// Unique identifier for the linked chat, i.e. the discussion group
    /// identifier for a channel and vice versa; for supergroups and channel
    /// chats. Returned only in [`get_chat`].
    ///
    /// [`get_chat`]: ../../api/trait.API.html#method.get_chat
    pub linked_chat_id: Option<i64>,
}

/// This object represents a chat. It can be a private, group, supergroup or
/// channel chat
#[allow(clippy::large_enum_variant)] // Using a box makes it more user-unfriendly
#[derive(Debug, Clone, PartialEq)]
pub enum Chat {
    Private(PrivateChat),
    Group(GroupChat),
    SuperGroup(SuperGroupChat),
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatPermissions {
    /// True, if the user is allowed to send text messages, contacts, locations
    /// and venues.
    #[serde(default)]
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios
    #[serde(default)]
    pub can_send_audios: bool,
    /// True, if the user is allowed to send documents
    #[serde(default)]
    pub can_send_documents: bool,
    /// True, if the user is allowed to send photos
    #[serde(default)]
    pub can_send_photos: bool,
    /// True, if the user is allowed to send videos
    #[serde(default)]
    pub can_send_videos: bool,
    /// True, if the user is allowed to send video notes
    #[serde(default)]
    pub can_send_video_notes: bool,
    /// True, if the user is allowed to send voice notes
    #[serde(default)]
    pub can_send_voice_notes: bool,
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
    /// True, if the user is allowed to create forum topics. If omitted defaults
    /// to the value of can_pin_messages
    #[serde(default)]
    pub can_manage_topics: bool,
}

/// This object represents a chat photo.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

    /// Gets the title of the chat, or username if it's a private chat
    /// In the possible case the user's username is unavailable, it is set to
    /// "unknown user"
    pub fn get_title(&self) -> &str {
        match self {
            Chat::Private(c) => c.username.as_ref().map_or("unknown user", String::as_str),
            Chat::Channel(c) => &c.title,
            Chat::Group(c) => &c.title,
            Chat::SuperGroup(c) => &c.title,
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
                active_usernames: raw.active_usernames,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                has_hidden_members: raw.has_hidden_members,
                has_protected_content: raw.has_protected_content,
                linked_chat_id: raw.linked_chat_id,
            }),
            ChatType::Private => Chat::Private(PrivateChat {
                id: raw.id,
                first_name: raw.first_name,
                last_name: raw.last_name,
                username: raw.username,
                photo: raw.photo,
                active_usernames: raw.active_usernames,
                emoji_status_custom_emoji_id: raw.emoji_status_custom_emoji_id,
                emoji_status_expiration_date: raw.emoji_status_expiration_date,
                bio: raw.bio,
                has_restricted_voice_and_video_messages: raw
                    .has_restricted_voice_and_video_messages,
                has_private_forwards: raw.has_private_forwards,
                message_auto_delete_time: raw.message_auto_delete_time,
            }),
            ChatType::Group => Chat::Group(GroupChat {
                id: raw.id,
                title: raw.title.unwrap_or_default(),
                photo: raw.photo,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                permissions: raw.permissions,
                has_hidden_members: raw.has_hidden_members,
                has_protected_content: raw.has_protected_content,
            }),
            ChatType::SuperGroup => Chat::SuperGroup(SuperGroupChat {
                id: raw.id,
                title: raw.title.unwrap_or_default(),
                username: raw.username,
                is_forum: raw.is_forum,
                photo: raw.photo,
                active_usernames: raw.active_usernames,
                join_by_request: raw.join_by_request,
                join_to_send_messages: raw.join_to_send_messages,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                permissions: raw.permissions,
                has_aggressive_anti_spam_enabled: raw.has_aggressive_anti_spam_enabled,
                has_hidden_members: raw.has_hidden_members,
                has_protected_content: raw.has_protected_content,
                sticker_set_name: raw.sticker_set_name,
                can_set_sticker_set: raw.can_set_sticker_set,
                slow_mode_delay: raw.slow_mode_delay,
                linked_chat_id: raw.linked_chat_id,
                location: raw.location,
            }),
            ChatType::Sender => unreachable!(),
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
                active_usernames: c.active_usernames,
                emoji_status_custom_emoji_id: c.emoji_status_custom_emoji_id,
                emoji_status_expiration_date: c.emoji_status_expiration_date,
                bio: c.bio,
                has_private_forwards: c.has_private_forwards,
                message_auto_delete_time: c.message_auto_delete_time,
                has_restricted_voice_and_video_messages: c.has_restricted_voice_and_video_messages,
                join_to_send_messages: false,
                join_by_request: false,
                title: None,
                description: None,
                pinned_message: None,
                invite_link: None,
                permissions: None,
                has_aggressive_anti_spam_enabled: false,
                has_hidden_members: false,
                has_protected_content: false,
                sticker_set_name: None,
                can_set_sticker_set: false,
                slow_mode_delay: None,
                linked_chat_id: None,
                location: None,
                is_forum: false,
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
                has_hidden_members: c.has_hidden_members,
                has_protected_content: c.has_protected_content,
                username: None,
                message_auto_delete_time: None,
                sticker_set_name: None,
                can_set_sticker_set: false,
                slow_mode_delay: None,
                first_name: None,
                last_name: None,
                bio: None,
                has_private_forwards: false,
                linked_chat_id: None,
                location: None,
                has_restricted_voice_and_video_messages: None,
                has_aggressive_anti_spam_enabled: false,
                join_to_send_messages: false,
                join_by_request: false,
                is_forum: false,
                active_usernames: Vec::new(),
                emoji_status_custom_emoji_id: None,
                emoji_status_expiration_date: None,
            },
            Chat::SuperGroup(c) => RawChat {
                chat_type: ChatType::SuperGroup,
                id: c.id,
                title: Some(c.title),
                username: c.username,
                photo: c.photo,
                active_usernames: c.active_usernames,
                description: c.description,
                pinned_message: c.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: c.invite_link,
                permissions: c.permissions,
                has_aggressive_anti_spam_enabled: c.has_aggressive_anti_spam_enabled,
                has_hidden_members: c.has_hidden_members,
                has_protected_content: c.has_protected_content,
                sticker_set_name: c.sticker_set_name,
                can_set_sticker_set: c.can_set_sticker_set,
                slow_mode_delay: c.slow_mode_delay,
                linked_chat_id: c.linked_chat_id,
                location: c.location,
                join_to_send_messages: c.join_to_send_messages,
                join_by_request: c.join_by_request,
                is_forum: c.is_forum,
                has_restricted_voice_and_video_messages: None,
                bio: None,
                has_private_forwards: false,
                first_name: None,
                last_name: None,
                message_auto_delete_time: None,
                emoji_status_custom_emoji_id: None,
                emoji_status_expiration_date: None,
            },
            Chat::Channel(c) => RawChat {
                chat_type: ChatType::Channel,
                id: c.id,
                title: Some(c.title),
                username: c.username,
                photo: c.photo,
                active_usernames: c.active_usernames,
                description: c.description,
                pinned_message: c.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: c.invite_link,
                has_hidden_members: c.has_hidden_members,
                has_protected_content: c.has_protected_content,
                linked_chat_id: c.linked_chat_id,
                permissions: None,
                message_auto_delete_time: None,
                sticker_set_name: None,
                can_set_sticker_set: false,
                slow_mode_delay: None,
                first_name: None,
                last_name: None,
                bio: None,
                has_private_forwards: false,
                location: None,
                has_aggressive_anti_spam_enabled: false,
                has_restricted_voice_and_video_messages: None,
                join_to_send_messages: false,
                join_by_request: false,
                is_forum: false,
                emoji_status_custom_emoji_id: None,
                emoji_status_expiration_date: None,
            },
        }
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawChat = Deserialize::deserialize(deserializer)?;

        Ok(raw.into())
    }
}

impl Serialize for Chat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawChat::from(self.clone()).serialize(serializer)
    }
}

/// This object contains information about one member of a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

/// Represents a [`ChatMember`] who is the creator or owner of the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    /// True, if the administrator can manage video chats
    #[serde(default)]
    pub can_manage_video_chats: bool,
    /// True, if the user is allowed to create, rename, close, and reopen forum
    /// topics; supergroups only
    #[serde(default)]
    pub can_manage_topics: bool,
}

/// Represents a [`ChatMember`] who is a normal member of the [`Chat`] without
/// any special powers.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MemberMemberStatus {
    /// Information about the user
    pub user: User,
}

/// Represents a restricted [`ChatMember`] of a [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RestrictedMemberStatus {
    /// Information about the user
    pub user: User,
    /// True, if the user is a member of the chat at the moment of the request
    #[serde(default)]
    pub is_member: bool,
    /// True, if the user is allowed to send text messages, contacts, locations
    /// and venues
    #[serde(default)]
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios
    #[serde(default)]
    pub can_send_audios: bool,
    /// True, if the user is allowed to send documents
    #[serde(default)]
    pub can_send_documents: bool,
    /// True, if the user is allowed to send photos
    #[serde(default)]
    pub can_send_photos: bool,
    /// True, if the user is allowed to send videos
    #[serde(default)]
    pub can_send_videos: bool,
    /// True, if the user is allowed to send video notes
    #[serde(default)]
    pub can_send_video_notes: bool,
    /// True, if the user is allowed to send voice notes
    #[serde(default)]
    pub can_send_voice_notes: bool,
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
    #[serde(default)]
    /// True, if the user is allowed to change the chat title, photo and other
    /// settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    #[serde(default)]
    pub can_invite_users: bool,
    /// True, if the user is allowed to pin messages; groups and supergroups
    /// only
    #[serde(default)]
    pub can_pin_messages: bool,
    /// True, if the user is allowed to create forum topics
    #[serde(default)]
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; unix time
    #[serde(with = "unix_date_formatting::optional")]
    pub until_date: Option<DateTime<Utc>>,
}

/// Represents a [`ChatMember`] who left the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LeftMemberStatus {
    /// Information about the user
    pub user: User,
}

/// Represents a [`ChatMember`] who has been kicked from the [`Chat`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    /// If users joining the chat via the link need to be approved by chat
    /// administrators
    pub creates_join_request: bool,
    /// Invite link name
    #[serde(default)]
    pub name: Option<String>,
    /// When the link will expire or has been expired
    #[serde(with = "unix_date_formatting::optional")]
    pub expire_date: Option<DateTime<Utc>>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    #[serde(default)]
    pub member_limit: Option<i32>,
    /// Number of pending join requests created using this link
    #[serde(default)]
    pub pending_join_request_count: Option<i32>,
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
    /// True, if the user joined the chat via a chat folder invite link
    #[serde(default)]
    pub via_chat_folder_invite_link: bool,
}

/// The type of chat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ChatType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    SuperGroup,
    #[serde(rename = "channel")]
    Channel,
    #[serde(rename = "sender")]
    Sender,
}

/// Represents a join request sent to a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request.
    /// The bot can use this identifier for 24 hours to send messages until the
    /// join request is processed, assuming no other administrator contacted the
    /// user.
    pub user_chat_id: i64,
    /// Date the request was sent in Unix time.
    #[serde(with = "unix_date_formatting")]
    pub date: DateTime<Utc>,
    /// Bio of the user.
    #[serde(default)]
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    #[serde(default)]
    pub invite_link: Option<ChatInviteLink>,
}

/// Represents the rights of an administrator in a chat.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, chat
    /// statistics, message statistics in channels, see channel members, see
    /// anonymous administrators in supergroups and ignore slow mode. Implied by
    /// any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of
    /// their own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed
    /// by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other
    /// settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// True, if the administrator can edit messages of other users and can pin
    /// messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// True, if the user is allowed to pin messages; groups and supergroups
    /// only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// True, if the user is allowed to create, rename, close, and reopen forum
    /// topics; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

/// This object represents a forum topic.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<String>,
}
