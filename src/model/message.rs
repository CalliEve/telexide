use super::utils::unix_date_formatting;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Game, Invoice, PassportData, Sticker, SuccessfulPayment, User};

pub use super::{message_contents::*, message_entity::*, InlineKeyboardMarkup};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawMessage {
    pub message_id: i64,
    pub from: Option<super::User>,
    #[serde(with = "unix_date_formatting")]
    pub date: DateTime<Utc>,
    pub chat: super::chat::RawChat,

    pub forward_from: Option<super::User>,
    pub forward_from_chat: Option<super::chat::RawChat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    #[serde(default)]
    #[serde(with = "unix_date_formatting::optional")]
    pub forward_date: Option<DateTime<Utc>>,

    pub reply_to_message: Option<Box<RawMessage>>,

    #[serde(default)]
    #[serde(with = "unix_date_formatting::optional")]
    pub edit_date: Option<DateTime<Utc>>,

    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,

    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub animation: Option<Animation>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub poll: Option<Poll>,
    pub dice: Option<Dice>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    #[serde(default)]
    pub delete_chat_photo: bool,
    #[serde(default)]
    pub group_chat_created: bool,
    #[serde(default)]
    pub supergroup_chat_created: bool,
    #[serde(default)]
    pub channel_chat_created: bool,

    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,

    pub pinned_message: Option<Box<RawMessage>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,

    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<super::User>,
    pub date: DateTime<Utc>,
    pub chat: super::Chat,

    pub forward_data: Option<ForwardData>,

    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<DateTime<Utc>>,
    pub author_signature: Option<String>,

    pub content: MessageContent,

    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum MessageContent {
    Text {
        content: String,
        entities: Vec<MessageEntity>,
    },
    Audio {
        content: Audio,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Document {
        content: Document,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Animation {
        content: Animation,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Video {
        content: Video,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        media_group_id: Option<String>,
    },
    Voice {
        content: Voice,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Photo {
        content: Vec<PhotoSize>,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        media_group_id: Option<String>,
    },

    Game {
        content: Game,
    },
    Sticker {
        content: Sticker,
    },
    VideoNote {
        content: VideoNote,
    },
    Contact {
        content: Contact,
    },
    Location {
        content: Location,
    },
    Venue {
        content: Venue,
    },
    Poll {
        content: Poll,
    },
    Dice {
        content: Dice,
    },
    NewChatMembers {
        content: Vec<User>,
    },
    LeftChatMember {
        content: User,
    },
    NewChatTitle {
        content: String,
    },
    NewChatPhoto {
        content: Vec<PhotoSize>,
    },
    MigrateToChatID {
        content: i64,
    },
    MigrateFromChatID {
        content: i64,
    },
    PinnedMessage {
        content: Box<Message>,
    },
    Invoice {
        content: Invoice,
    },
    SuccessfulPayment {
        content: SuccessfulPayment,
    },

    DeleteChatPhoto,
    GroupChatCreated,
    SupergroupChatCreated,
    ChannelChatCreated,

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForwardData {
    pub from: Option<super::User>,
    pub from_chat: Option<super::Chat>,
    pub from_message_id: Option<i64>,
    pub signature: Option<String>,
    pub sender_name: Option<String>,
    pub date: DateTime<Utc>,
}

impl Message {
    pub fn get_text(&self) -> Option<String> {
        match self.content {
            MessageContent::Text {
                ref content, ..
            } => Some(content.clone()),
            MessageContent::Audio {
                ref caption, ..
            } | MessageContent::Document {
                ref caption, ..
            } | MessageContent::Animation {
                ref caption, ..
            } | MessageContent::Video {
                ref caption, ..
            } | MessageContent::Voice {
                ref caption, ..
            } | MessageContent::Photo {
                ref caption, ..
            } => caption.clone(),
            _ => None,
        }
    }
}

impl From<RawMessage> for Message {
    #[allow(clippy::too_many_lines)]
    fn from(raw: RawMessage) -> Message {
        let message_id = raw.message_id;
        let from = raw.from;
        let date = raw.date;
        let chat = raw.chat.into();
        let reply_to_message = raw.reply_to_message.map(|r| Box::new((*r).into()));
        let edit_date = raw.edit_date;
        let author_signature = raw.author_signature;
        let connected_website = raw.connected_website;
        let passport_data = raw.passport_data;
        let reply_markup = raw.reply_markup;

        let forward_data = if let Some(d) = raw.forward_date {
            Some(ForwardData {
                from: raw.forward_from,
                from_chat: raw.forward_from_chat.map(|c| c.into()),
                from_message_id: raw.forward_from_message_id,
                signature: raw.forward_signature,
                sender_name: raw.forward_sender_name,
                date: d,
            })
        } else {
            None
        };

        let fill_in_content = |content: MessageContent| {
            Self {
                message_id,
                from,
                date,
                chat,
                forward_data,
                reply_to_message,
                edit_date,
                author_signature,
                content,
                connected_website,
                passport_data,
                reply_markup,
            }
        };

        if let Some(c) = raw.text {
            return fill_in_content(MessageContent::Text {
                content: c,
                entities: raw.entities.unwrap_or_default(),
            });
        } else if let Some(c) = raw.video {
            return fill_in_content(MessageContent::Video {
                content: c,
                caption: raw.caption,
                caption_entities: raw.caption_entities,
                media_group_id: raw.media_group_id,
            });
        } else if let Some(c) = raw.photo {
            return fill_in_content(MessageContent::Photo {
                content: c,
                caption: raw.caption,
                caption_entities: raw.caption_entities,
                media_group_id: raw.media_group_id,
            });
        } else if let Some(c) = raw.pinned_message {
            return fill_in_content(MessageContent::PinnedMessage {
                content: Box::new((*c).into()),
            });
        }

        macro_rules! content_with_captions {
            ($data:expr, $kind:ident) => {
                if let Some(c) = $data {
                    return fill_in_content(MessageContent::$kind {
                        content: c,
                        caption: raw.caption,
                        caption_entities: raw.caption_entities,
                    });
                }
            };
        }

        macro_rules! content {
            ($data:expr, $kind:ident) => {
                if let Some(c) = $data {
                    return fill_in_content(MessageContent::$kind {
                        content: c,
                    });
                }
            };
        }

        macro_rules! bool_content {
            ($data:expr, $kind:ident) => {
                if $data {
                    return fill_in_content(MessageContent::$kind);
                }
            };
        }

        content_with_captions!(raw.audio, Audio);
        content_with_captions!(raw.document, Document);
        content_with_captions!(raw.animation, Animation);
        content_with_captions!(raw.voice, Voice);

        content!(raw.game, Game);
        content!(raw.sticker, Sticker);
        content!(raw.video_note, VideoNote);
        content!(raw.contact, Contact);
        content!(raw.location, Location);
        content!(raw.venue, Venue);
        content!(raw.poll, Poll);
        content!(raw.dice, Dice);
        content!(raw.new_chat_members, NewChatMembers);
        content!(raw.left_chat_member, LeftChatMember);
        content!(raw.new_chat_title, NewChatTitle);
        content!(raw.new_chat_photo, NewChatPhoto);
        content!(raw.migrate_to_chat_id, MigrateToChatID);
        content!(raw.migrate_from_chat_id, MigrateFromChatID);
        content!(raw.invoice, Invoice);
        content!(raw.successful_payment, SuccessfulPayment);

        bool_content!(raw.delete_chat_photo, DeleteChatPhoto);
        bool_content!(raw.group_chat_created, GroupChatCreated);
        bool_content!(raw.supergroup_chat_created, SupergroupChatCreated);
        bool_content!(raw.channel_chat_created, ChannelChatCreated);

        fill_in_content(MessageContent::Unknown)
    }
}

impl From<Message> for RawMessage {
    #[allow(clippy::too_many_lines)]
    fn from(message: Message) -> RawMessage {
        let mut ret = Self {
            message_id: message.message_id,
            from: message.from,
            date: message.date,
            chat: message.chat.into(),
            reply_to_message: message.reply_to_message.map(|r| Box::new((*r).into())),
            edit_date: message.edit_date,
            media_group_id: None,
            author_signature: message.author_signature,

            forward_date: None,
            forward_sender_name: None,
            forward_signature: None,
            forward_from_message_id: None,
            forward_from: None,
            forward_from_chat: None,

            text: None,
            entities: None,
            caption_entities: None,
            audio: None,
            document: None,
            animation: None,
            game: None,
            photo: None,
            sticker: None,
            video: None,
            voice: None,
            video_note: None,
            caption: None,
            contact: None,
            location: None,
            venue: None,
            poll: None,
            dice: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: false,
            group_chat_created: false,
            supergroup_chat_created: false,
            channel_chat_created: false,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,

            connected_website: message.connected_website,
            passport_data: message.passport_data,
            reply_markup: message.reply_markup,
        };

        if let Some(d) = message.forward_data {
            ret.forward_date = Some(d.date);
            ret.forward_sender_name = d.sender_name;
            ret.forward_signature = d.signature;
            ret.forward_from_message_id = d.from_message_id;
            ret.forward_from = d.from;
            ret.forward_from_chat = d.from_chat.map(|c| c.into());
        }

        match message.content {
            MessageContent::Text {
                content,
                entities,
            } => {
                ret.text = Some(content);
                ret.entities = Some(entities);
                ret
            },
            MessageContent::Audio {
                content,
                caption,
                caption_entities,
            } => {
                ret.audio = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret
            },
            MessageContent::Document {
                content,
                caption,
                caption_entities,
            } => {
                ret.document = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret
            },
            MessageContent::Animation {
                content,
                caption,
                caption_entities,
            } => {
                ret.animation = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret
            },
            MessageContent::Voice {
                content,
                caption,
                caption_entities,
            } => {
                ret.voice = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret
            },
            MessageContent::Video {
                content,
                caption,
                caption_entities,
                media_group_id,
            } => {
                ret.video = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret.media_group_id = media_group_id;
                ret
            },
            MessageContent::Photo {
                content,
                caption,
                caption_entities,
                media_group_id,
            } => {
                ret.photo = Some(content);
                ret.caption = caption;
                ret.caption_entities = caption_entities;
                ret.media_group_id = media_group_id;
                ret
            },
            MessageContent::Game {
                content,
            } => {
                ret.game = Some(content);
                ret
            },
            MessageContent::Sticker {
                content,
            } => {
                ret.sticker = Some(content);
                ret
            },
            MessageContent::VideoNote {
                content,
            } => {
                ret.video_note = Some(content);
                ret
            },
            MessageContent::Contact {
                content,
            } => {
                ret.contact = Some(content);
                ret
            },
            MessageContent::Location {
                content,
            } => {
                ret.location = Some(content);
                ret
            },
            MessageContent::Venue {
                content,
            } => {
                ret.venue = Some(content);
                ret
            },
            MessageContent::Poll {
                content,
            } => {
                ret.poll = Some(content);
                ret
            },
            MessageContent::Dice {
                content,
            } => {
                ret.dice = Some(content);
                ret
            },
            MessageContent::NewChatMembers {
                content,
            } => {
                ret.new_chat_members = Some(content);
                ret
            },
            MessageContent::LeftChatMember {
                content,
            } => {
                ret.left_chat_member = Some(content);
                ret
            },
            MessageContent::NewChatTitle {
                content,
            } => {
                ret.new_chat_title = Some(content);
                ret
            },
            MessageContent::NewChatPhoto {
                content,
            } => {
                ret.new_chat_photo = Some(content);
                ret
            },
            MessageContent::MigrateToChatID {
                content,
            } => {
                ret.migrate_to_chat_id = Some(content);
                ret
            },
            MessageContent::MigrateFromChatID {
                content,
            } => {
                ret.migrate_from_chat_id = Some(content);
                ret
            },
            MessageContent::Invoice {
                content,
            } => {
                ret.invoice = Some(content);
                ret
            },
            MessageContent::SuccessfulPayment {
                content,
            } => {
                ret.successful_payment = Some(content);
                ret
            },
            MessageContent::PinnedMessage {
                content,
            } => {
                ret.pinned_message = Some(Box::new((*content).into()));
                ret
            },
            MessageContent::DeleteChatPhoto => {
                ret.delete_chat_photo = true;
                ret
            },
            MessageContent::GroupChatCreated => {
                ret.group_chat_created = true;
                ret
            },
            MessageContent::SupergroupChatCreated => {
                ret.supergroup_chat_created = true;
                ret
            },
            MessageContent::ChannelChatCreated => {
                ret.channel_chat_created = true;
                ret
            },
            _ => ret,
        }
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Ok(raw.into())
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawMessage::from(self.to_owned()).serialize(serializer)
    }
}
