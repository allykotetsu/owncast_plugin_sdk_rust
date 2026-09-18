use anyhow::{anyhow, Error};
use crate::json_objects::sql_exec_result::SqlExecResult;

/// The response to an Ok [`owncast::sql::exec`].
#[derive(Clone, Debug)]
pub struct PartialSqlExecResult {
    /// How many rows were modified because of the SQL execution.
    pub rows_affected: i64,

    /// The ID of the last row inserted because of the SQL execution.
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