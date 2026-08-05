use extism_pdk::SharedFnResult;
use crate::host::{owncast_sql_exec, owncast_sql_query};
use crate::json_objects::partial_sql_exec_result::PartialSqlExecResult;
use crate::json_objects::partial_sql_query_result::PartialSqlQueryResult;
use crate::json_objects::sql_request::SqlRequest;
use crate::json_objects::sql_row::SqlRow;
use crate::json_objects::sql_value::SqlValue;

pub fn exec(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<PartialSqlExecResult> {
    let req = &SqlRequest::from((sql, params.clone(), None));
    let res = unsafe {
        owncast_sql_exec(req)
    };
    res?.try_into()
}

fn do_query(sql: &str, params: &Vec<SqlValue>, max_rows: Option<i64>) -> SharedFnResult<PartialSqlQueryResult> {
    let req = &SqlRequest::from((sql, params.clone(), max_rows));
    let res = unsafe {
        owncast_sql_query(req)
    };
    res?.try_into()
}

pub fn query(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<Vec<SqlRow>> {
    do_query(sql, params, None)?.try_into()
}

pub fn query_row(sql: &str, params: &Vec<SqlValue>) -> SharedFnResult<Option<SqlRow>> {
    let rows: Vec<SqlRow> = do_query(sql, params, Some(1))?.try_into()?;
    Ok(rows.get(0).cloned())
}