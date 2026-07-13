use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StreamStarted {
    started_at: String,
    title: String,
    summary: String
}