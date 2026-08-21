use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::autoplay_mode::AutoplayMode;
use crate::json_objects::stream_variant::StreamVariant;
use crate::json_objects::video_codec::VideoCodec;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfigUpdate {
    pub latency_level: Option<i64>,
    pub codec: Option<VideoCodec>,
    pub autoplay: Option<AutoplayMode>,
    pub variants: Option<Vec<StreamVariant>>
}

impl VideoConfigUpdate {
    pub fn new() -> Self {
        Self {
            latency_level: None,
            codec: None,
            autoplay: None,
            variants: None,
        }
    }

    pub fn with_latency_level(mut self, latency_level: i64) -> Self {
        self.latency_level = Some(latency_level);
        self
    }

    pub fn with_codec(mut self, codec: VideoCodec) -> Self {
        self.codec = Some(codec);
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