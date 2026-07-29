use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamInfo {
    pub online: bool,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub viewers: i64,
    pub started_at: Option<String>,
    pub latency_level: Option<i64>
}

impl StreamInfo {
    pub(crate) fn offline() -> Self {
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