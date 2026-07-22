use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamStarted {
    started_at: String,
    title: String,
    summary: String
}