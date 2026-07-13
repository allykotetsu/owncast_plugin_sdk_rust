use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StreamTitleChange {
    pub(crate) from: String,
    pub(crate) to: String
}