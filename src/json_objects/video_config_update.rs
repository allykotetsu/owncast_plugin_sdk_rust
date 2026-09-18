use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::autoplay_mode::AutoplayMode;
use crate::json_objects::stream_variant::StreamVariant;
use crate::json_objects::video_codec::VideoCodec;

/// An object containing data used to update Owncast's video config.
#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfigUpdate {
    /// The latency level Owncast should have.
    pub latency_level: Option<i64>,

    /// The [`VideoCodec`] Owncast should use.
    pub codec: Option<VideoCodec>,

    /// What autoplay mode Owncast should use.
    pub autoplay: Option<AutoplayMode>,

    /// The stream variants Owncast should have.
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