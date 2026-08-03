use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::stream_variant::StreamVariant;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfigUpdate {
    pub latency_level: Option<i64>,
    pub codec: Option<String>,
    pub variants: Option<Vec<StreamVariant>>
}

impl VideoConfigUpdate {
    pub fn new() -> Self {
        Self {
            latency_level: None,
            codec: None,
            variants: None,
        }
    }

    pub fn with_latency_level(mut self, latency_level: i64) -> Self {
        self.latency_level = Some(latency_level);
        self
    }

    pub fn with_codec(mut self, codec: &str) -> Self {
        self.codec = Some(codec.to_string());
        self
    }

    pub fn with_variant(mut self, stream_variant: StreamVariant) -> Self {
        if let Some(ref mut entries) = self.variants {
            entries.push(stream_variant);
        } else {
            self.variants = Some(vec![stream_variant]);
        }
        self
    }
}