use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct SqlExecResult {
    pub error: Option<String>,
    pub rows_affected: i64,
    pub last_insert_id: i64
}