use serde::Deserialize;

/// An object representing an attachment on the fediverse.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// The URL of the resource the [`Attachment`] is displaying.
    pub url: String,

    /// The media type of the [`Attachment`].
    pub media_type: String,

    /// The alt text of the [`Attachment`].
    pub alt: Option<String>,
}