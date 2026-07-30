use extism_pdk::{ToBytes, FromBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromBytes, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct StreamVariant {
    pub width: i64,
    pub height: i64,
    pub framerate: i64,
    pub video_bitrate: i64,
    pub audio_bitrate: i64,
    pub is_passthrough: bool
}