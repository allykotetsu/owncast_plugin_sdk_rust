use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamStarted {
    pub started_at: String,
    pub title: String,
    pub summary: String
}