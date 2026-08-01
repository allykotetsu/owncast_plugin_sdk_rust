use anyhow::{anyhow, Error};
use crate::json_objects::sql_query_result::SqlQueryResult;
use crate::json_objects::sql_value::SqlValue;

#[derive(Clone, Debug)]
pub struct PartialSqlQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<SqlValue>>,
    pub truncated: Option<bool>
}

impl TryFrom<SqlQueryResult> for PartialSqlQueryResult {
    type Error = Error;

    fn try_from(sql_query_result: SqlQueryResult) -> Result<Self, Self::Error> {
        if let Some(error) = sql_query_result.error {
            Err(anyhow!(error))
        } else {
            Ok(Self {
                columns: sql_query_result.columns,
                rows: sql_query_result.rows,
                truncated: sql_query_result.truncated,
            })
        }
    }
}