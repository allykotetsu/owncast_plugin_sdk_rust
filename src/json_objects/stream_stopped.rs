use serde::Deserialize;

/// [`Envelope::StreamStopped`] event payload.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamStopped {
    /// Timestamp (ISO-8601) for when the stream stopped.
    pub stopped_at: String
}