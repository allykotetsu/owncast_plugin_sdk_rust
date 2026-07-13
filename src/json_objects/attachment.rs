use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Attachment {
    pub(crate) url: String,
    pub(crate) media_type: String,
    pub(crate) alt: Option<String>,
}