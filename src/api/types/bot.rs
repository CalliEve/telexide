use crate::model::ChatAdministratorRights;
use serde::{Deserialize, Serialize};
use telexide_proc_macros::build_struct;

/// struct for holding data needed to call
/// [`set_my_default_administrator_rights`]
///
/// [`set_my_default_administrator_rights`]:
/// ../../api/trait.API.html#method.set_my_default_administrator_rights
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyDefaultAdministratorRights {
    /// A JSON-serialized object describing new default administrator rights. If
    /// not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    /// Pass True to change the default administrator rights of the bot in
    /// channels. Otherwise, the default administrator rights of the bot for
    /// groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<bool>,
}

/// struct for holding data needed to call
/// [`set_my_default_administrator_rights`]
///
/// [`set_my_default_administrator_rights`]:
/// ../../api/trait.API.html#method.set_my_default_administrator_rights
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyDefaultAdministratorRights {
    /// Pass True to get the default administrator rights of the bot in
    /// channels. Otherwise, the default administrator rights of the bot for
    /// groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<bool>,
}

/// struct for holding data needed to call
/// [`set_my_name`]
///
/// [`set_my_name`]:
/// ../../api/trait.API.html#method.set_my_name
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyName {
    /// New bot name; 0-64 characters. Pass an empty string to remove the
    /// dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown
    /// to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`get_my_name`]
///
/// [`get_my_name`]:
/// ../../api/trait.API.html#method.get_my_name
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyName {
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`set_my_description`]
///
/// [`set_my_description`]:
/// ../../api/trait.API.html#method.set_my_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyDescription {
    /// New bot description; 0-512 characters. Pass an empty string to remove
    /// the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be
    /// applied to all users for whose language there is no dedicated
    /// description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`get_my_description`]
///
/// [`get_my_description`]:
/// ../../api/trait.API.html#method.get_my_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyDescription {
    /// A two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`set_my_short_description`]
///
/// [`set_my_short_description`]:
/// ../../api/trait.API.html#method.set_my_short_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMyShortDescription {
    /// New bot description; 0-120 characters. Pass an empty string to remove
    /// the dedicated description for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the description will be
    /// applied to all users for whose language there is no dedicated
    /// description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// struct for holding data needed to call
/// [`get_my_short_description`]
///
/// [`get_my_short_description`]:
/// ../../api/trait.API.html#method.get_my_short_description
#[build_struct]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMyShortDescription {
    /// A two-letter ISO 639-1 language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
