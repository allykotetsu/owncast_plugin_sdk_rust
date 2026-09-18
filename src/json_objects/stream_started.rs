use serde::Deserialize;

/// [`Envelope::StreamStarted`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamStarted {
    /// Timestamp (ISO-8601) for when the stream started.
    pub started_at: String,

    /// The title of the stream.
    pub title: String,

    /// The "about" section of the stream.
    pub summary: String
}