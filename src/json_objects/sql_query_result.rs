use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::sql_value::SqlValue;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct SqlQueryResult {
    pub error: Option<String>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<SqlValue>>,
    pub truncated: Option<bool>
}