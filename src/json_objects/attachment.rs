use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub url: String,
    pub media_type: String,
    pub alt: Option<String>,
}