use anyhow::{anyhow, Error};
use crate::json_objects::sql_exec_result::SqlExecResult;

#[derive(Clone, Debug)]
pub struct PartialSqlExecResult {
    pub rows_affected: i64,
    pub last_insert_id: i64
}

impl TryFrom<SqlExecResult> for PartialSqlExecResult {
    type Error = Error;

    fn try_from(sql_exec_result: SqlExecResult) -> Result<Self, Self::Error> {
        if let Some(error) = sql_exec_result.error {
            Err(anyhow!(error))
        } else {
            Ok(Self {
                rows_affected: sql_exec_result.rows_affected,
                last_insert_id: sql_exec_result.last_insert_id,
            })
        }
    }
}