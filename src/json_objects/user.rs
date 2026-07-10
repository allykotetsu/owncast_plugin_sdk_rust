use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub display_color: u16,
    pub previous_names: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub disabled_at: Option<String>,
    pub is_bot: Option<bool>,
    pub is_authenticated: Option<bool>,
    pub scopes: Option<Vec<String>>
}