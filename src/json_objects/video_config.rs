use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::stream_variant::StreamVariant;

#[derive(Deserialize, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfig {
    pub latency_level: i64,
    pub codec: String,
    pub variants: Vec<StreamVariant>
}