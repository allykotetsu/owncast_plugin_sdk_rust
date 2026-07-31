use std::collections::HashMap;
use anyhow::Error;
use crate::json_objects::sql_query_result::SqlQueryResult;
use crate::json_objects::sql_value::SqlValue;

pub type SqlRow = HashMap<String, SqlValue>;

impl TryFrom<SqlQueryResult> for Vec<SqlRow> {
    type Error = Error;

    fn try_from(sql_query_result: SqlQueryResult) -> Result<Self, Self::Error> {
        let collection: Vec<SqlRow> = sql_query_result.rows.iter().map(|values| {
            let collection: HashMap<String, SqlValue> = sql_query_result.columns.iter().map(|column| {
                // Find the index in the columns array of the given column.
                let i = sql_query_result.columns.iter().position(|k| k == column)?;
                // Return a tuple of the SQL column name and the SqlValue at the given index.
                Some((column.clone(), values.get(i)?.clone()))
            }).filter_map(|pair| pair).collect();
            collection
        }).collect();

        Ok(collection)
    }
}