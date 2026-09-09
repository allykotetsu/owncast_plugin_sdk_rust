use extism_pdk::SharedFnResult;
use serde::de::DeserializeOwned;
use serde_json::Error;
use crate::host::{owncast_sql_exec, owncast_sql_query};
use crate::json_objects::partial_sql_exec_result::PartialSqlExecResult;
use crate::json_objects::partial_sql_query_result::PartialSqlQueryResult;
use crate::json_objects::sql_request::SqlRequest;
use crate::json_objects::sql_row::SqlRow;
use crate::json_objects::sql_value::SqlValue;

pub fn exec(sql: &str, params: impl Into<Vec<SqlValue>>) -> SharedFnResult<PartialSqlExecResult> {
    let req = &SqlRequest::from((sql, params.into(), None));
    let res = unsafe {
        owncast_sql_exec(req)
    };
    res?.try_into()
}

fn do_query(sql: &str, params: Vec<SqlValue>, max_rows: Option<i64>) -> SharedFnResult<PartialSqlQueryResult> {
    let req = &SqlRequest::from((sql, params.clone(), max_rows));
    let res = unsafe {
        owncast_sql_query(req)
    };
    res?.try_into()
}

pub fn query<T: DeserializeOwned>(sql: &str, params: impl Into<Vec<SqlValue>>) -> SharedFnResult<Vec<Result<T, Error>>> {
    let rows: Vec<SqlRow> = do_query(sql, params.into(), None)?.try_into()?;

    let rows = rows.iter()
        .map(|sql_row| Ok(serde_json::from_value(serde_json::to_value(sql_row)?)?))
        .collect::<Vec<Result<T, Error>>>();

    Ok(rows.try_into()?)
}

pub fn query_row<T: DeserializeOwned>(sql: &str, params: impl Into<Vec<SqlValue>>) -> SharedFnResult<Option<Result<T, Error>>> {
    let rows: Vec<SqlRow> = do_query(sql, params.into(), Some(1))?.try_into()?;

    Ok(rows.get(0).map(|sql_row| Ok(serde_json::from_value(serde_json::to_value(sql_row)?)?)))
}