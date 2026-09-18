use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::autoplay_mode::AutoplayMode;
use crate::json_objects::stream_variant::StreamVariant;

/// An object representing Owncast video configuration information.
#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfig {
    /// The level of latency Owncast has.
    pub latency_level: i64,

    /// The video codec Owncast uses.
    pub codec: String,

    /// How to autoplay stream when the page is loaded.
    pub autoplay: AutoplayMode,

    /// Stream quality variants.
    pub variants: Vec<StreamVariant>
}