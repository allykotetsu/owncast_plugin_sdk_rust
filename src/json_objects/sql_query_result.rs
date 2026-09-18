use extism_pdk::{FromBytes, Json};
use serde::Deserialize;
use crate::json_objects::sql_value::SqlValue;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub(crate) struct SqlQueryResult {
    pub(crate) error: Option<String>,
    pub(crate) columns: Vec<String>,
    pub(crate) rows: Vec<Vec<SqlValue>>,
    pub(crate) truncated: Option<bool>
}