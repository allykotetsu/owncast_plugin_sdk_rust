use extism_pdk::{ToBytes, FromBytes, Json};
use serde::{Deserialize, Serialize};

/// An object representing a variant of video output that stream viewers can pick from.
#[derive(Deserialize, Serialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamVariant {
    /// The width in pixels of the video output.
    pub width: i64,

    /// The height in pixels of the video output.
    pub height: i64,

    /// The framerate in frames per second of the video output.
    pub framerate: i64,

    /// The bitrate of the video output.
    pub video_bitrate: i64,

    /// The allowed CPU usage of the video output.
    pub cpu_usage_level: i64,

    /// Whether the video output is passthrough.
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