use super::{Animation, MessageEntity, PhotoSize, User};
use serde::{Deserialize, Serialize};

/// This object represents a game. Use [@BotFather](https://t.me/botfather) to create and edit games,
/// their short names will act as unique identifiers.
#[allow(clippy::doc_markdown)] // BotFather is a thing
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game
    /// message. Can be automatically edited to include current high scores
    /// for the game when the bot calls [set_game_score], or manually edited
    /// using [edit_message_text]. 0-4096 characters.
    ///
    /// [edit_message_text]: ../api/trait.API.html#method.edit_message_text
    /// [set_game_score]: ../api/trait.API.html#method.set_game_score
    pub text: Option<String>,
    /// Special entities that appear in text, such as usernames, URLs, bot
    /// commands, etc.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats. Upload
    /// via BotFather
    pub animation: Option<Animation>,
}

/// A placeholder, currently holds no information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CallbackGame {}

/// This object represents one row of the high scores table for a game.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// The User
    pub user: User,
    /// Score
    pub score: i64,
}
