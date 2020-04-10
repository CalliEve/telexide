use super::{Animation, MessageEntity, PhotoSize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    pub text: Option<String>,
    pub text_entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
}

// A placeholder, currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct CallbackGame {}
