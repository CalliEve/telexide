use super::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: usize,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub duration: usize,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: usize,
    pub height: usize,
    pub duration: usize,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: usize,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: usize,
    pub duration: usize,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<i64>,
    pub vcard: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Poll {
    pub id: String,
    pub question: String,
    pub options: Vec<PollOption>,
    pub total_voter_count: usize,
    #[serde(default)]
    pub is_closed: bool,
    #[serde(default)]
    pub is_anonymous: bool,
    #[serde(default)]
    pub allows_multiple_answers: bool,
    #[serde(rename = "type")]
    pub poll_type: PollType,
    pub correct_option_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Dice {
    pub value: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PollOption {
    pub text: String,
    pub voter_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PollAnswer {
    pub poll_id: String,
    pub user: User,
    pub option_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum PollType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "quiz")]
    Quiz,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct LoginUrl {
    pub url: String,
    pub forward_text: Option<String>,
    pub bot_username: Option<String>,
    #[serde(default)]
    pub request_write_access: bool,
}
