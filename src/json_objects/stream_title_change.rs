use serde::Deserialize;

/// [`Envelope::StreamTitleChanged`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamTitleChange {
    /// The previous title that the stream had.
    pub from: String,

    /// The new title that the stream has.
    pub to: String
}