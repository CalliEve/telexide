use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct RawChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    // Title, for supergroups, channels and group chats
    pub title: Option<String>,
    // Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    // First name of the other party in a private chat
    pub first_name: Option<String>,
    // Last name of the other party in a private chat
    pub last_name: Option<String>,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    // Description, for groups, supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    // Chat invite link, for groups, supergroups and channel chats.
    pub invite_link: Option<String>,
    // Pinned message, for groups, supergroups and channels. Returned only in getChat.
    pub pinned_message: Option<Box<super::message::RawMessage>>,
    // Default chat member permissions, for groups and supergroups. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
    // For supergroups, the minimum allowed delay between consecutive messages sent by each
    // unpriviledged user. Returned only in getChat.
    pub slow_mode_delay: Option<usize>,
    // For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    // True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PrivateChat {
    pub id: i64,
    // Username if available
    pub username: Option<String>,
    // First name of the other party
    pub first_name: Option<String>,
    // Last name of the other party
    pub last_name: Option<String>,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct GroupChat {
    pub id: i64,
    // Title
    pub title: String,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    // Description. Returned only in getChat.
    pub description: Option<String>,
    // Chat invite link
    pub invite_link: Option<String>,
    // Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
    // Default chat member permissions. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct SuperGroupChat {
    pub id: i64,
    // Title
    pub title: String,
    // Username if available
    pub username: Option<String>,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    // Description. Returned only in getChat.
    pub description: Option<String>,
    // Chat invite link
    pub invite_link: Option<String>,
    // Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
    // Default chat member permissions. Returned only in getChat.
    pub permissions: Option<super::ChatPermissions>,
    // The minimum allowed delay between consecutive messages sent by each unpriviledged user.
    // Returned only in getChat.
    pub slow_mode_delay: Option<usize>,
    // Name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    // True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ChannelChat {
    pub id: i64,
    // Title
    pub title: String,
    // Username if available
    pub username: Option<String>,
    // Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    // Description. Returned only in getChat.
    pub description: Option<String>,
    // Chat invite link
    pub invite_link: Option<String>,
    // Pinned message. Returned only in getChat.
    pub pinned_message: Option<Box<super::Message>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
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
                title: raw.title.unwrap_or(String::default()),
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
                title: raw.title.unwrap_or(String::default()),
                photo: raw.photo,
                description: raw.description,
                pinned_message: raw.pinned_message.map(|m| Box::new((*m).into())),
                invite_link: raw.invite_link,
                permissions: raw.permissions,
            }),
            ChatType::SuperGroup => Chat::SuperGroup(SuperGroupChat {
                id: raw.id,
                title: raw.title.unwrap_or(String::default()),
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
