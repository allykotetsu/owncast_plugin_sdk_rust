use extism_pdk::{ToBytes, FromBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamVariant {
    pub width: i64,
    pub height: i64,
    pub framerate: i64,
    pub video_bitrate: i64,
    pub cpu_usage_level: i64,
    pub is_passthrough: bool
}

impl StreamVariant {
    pub fn new(width: i64, height: i64, framerate: i64, video_bitrate: i64, cpu_usage_level: i64, is_passthrough: bool) -> Self {
        Self {
            width,
            height,
            framerate,
            video_bitrate,
            cpu_usage_level,
            is_passthrough
        }
    }
}