use extism_pdk::SharedFnResult;
use crate::host::{owncast_sql_exec, owncast_sql_query};
use crate::json_objects::sql_exec_result::SqlExecResult;
use crate::json_objects::sql_query_result::SqlQueryResult;
use crate::json_objects::sql_request::SqlRequest;
use crate::json_objects::sql_row::SqlRow;
use crate::json_objects::sql_value::SqlValue;

// TODO move out error and remove OK
pub fn exec(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<SqlExecResult> {
    unsafe {
        owncast_sql_exec(&SqlRequest::from((sql, params.clone(), None)))
    }
}

// TODO move out error and remove OK
fn do_query(sql: &str, params: &Vec<SqlValue>, max_rows: Option<i64>) -> SharedFnResult<SqlQueryResult> {
    unsafe {
        owncast_sql_query(&SqlRequest::from((sql, params.clone(), max_rows)))
    }
}

pub fn query(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<Vec<SqlRow>> {
    Ok(do_query(sql, params, None)?.try_into()?)
}

pub fn query_row(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<Option<SqlRow>> {
    let rows: Vec<SqlRow> = do_query(sql, params, Some(1))?.try_into()?;
    Ok(match rows.get(0) {
        Some(row) => Some(row.clone()),
        None => None
    })
}