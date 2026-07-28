use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamTitleChange {
    pub from: String,
    pub to: String
}