use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

/// An object representing information about an instance of a stream.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamInfo {
    /// Whether stream is online or not.
    pub online: bool,

    /// The title of what is being streamed.
    pub title: Option<String>,

    /// The about section of the Owncast server.
    pub summary: Option<String>,

    /// How many users are watching stream.
    pub viewers: i64,

    /// The ISO-8601 timestamp for when the stream started.
    pub started_at: Option<String>,

    /// The latency level of stream.
    pub latency_level: Option<i64>
}

impl StreamInfo {
    pub fn offline() -> Self {
        StreamInfo {
            online: false,
            title: None,
            summary: None,
            viewers: 0,
            started_at: None,
            latency_level: None,
        }
    }
}