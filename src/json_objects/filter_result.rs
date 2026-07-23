use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(ToBytes, Serialize)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum FilterResult {
    Pass,
    Modify { payload: String },
    Drop { reason: String }
}