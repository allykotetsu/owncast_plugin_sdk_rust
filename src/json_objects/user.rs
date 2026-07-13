use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) display_name: String,
    pub(crate) display_color: u16,
    pub(crate) previous_names: Option<Vec<String>>,
    pub(crate) created_at: Option<String>,
    pub(crate) disabled_at: Option<String>,
    pub(crate) is_bot: Option<bool>,
    pub(crate) is_authenticated: Option<bool>,
    pub(crate) scopes: Option<Vec<String>>
}