use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub(crate) struct SqlExecResult {
    pub(crate) error: Option<String>,
    pub(crate) rows_affected: i64,
    pub(crate) last_insert_id: i64
}