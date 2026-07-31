use extism_pdk::{ToBytes, Json};
use serde::Serialize;

#[derive(ToBytes, Serialize, Clone, Debug)]
#[encoding(Json)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum AuthCheckResult {
    Ok,
    Refresh { ttl: Option<u64> },
    Deny { reason: String }
}