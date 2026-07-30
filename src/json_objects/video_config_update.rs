use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::stream_variant::StreamVariant;

#[derive(Serialize, ToBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct VideoConfigUpdate {
    pub latency_level: i64,
    pub codec: Option<String>,
    pub variants: Option<Vec<StreamVariant>>
}