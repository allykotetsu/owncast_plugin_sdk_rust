use extism_pdk::{ToBytes, Json};
use serde::Serialize;
use crate::json_objects::sql_value::SqlValue;

#[derive(Serialize, ToBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub(crate) struct SqlRequest {
    pub(crate) sql: String,
    pub(crate) params: Vec<SqlValue>,
    pub(crate) max_rows: Option<i64>
}

impl From<(&str, Vec<SqlValue>, Option<i64>)> for SqlRequest {
    fn from((sql, params, max_rows): (&str, Vec<SqlValue>, Option<i64>)) -> Self {
        Self {
            sql: sql.to_string(),
            params,
            max_rows
        }
    }
}