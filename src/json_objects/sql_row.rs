use std::collections::HashMap;
use anyhow::Error;
use crate::json_objects::partial_sql_query_result::PartialSqlQueryResult;
use crate::json_objects::sql_value::SqlValue;

pub type SqlRow = HashMap<String, SqlValue>;

impl TryFrom<PartialSqlQueryResult> for Vec<SqlRow> {
    type Error = Error;

    fn try_from(partial_sql_query_result: PartialSqlQueryResult) -> Result<Self, Self::Error> {
        let collection: Vec<SqlRow> = partial_sql_query_result.rows.iter().map(|values| {
            let collection: HashMap<String, SqlValue> = partial_sql_query_result.columns.iter().map(|column| {
                // Find the index in the columns array of the given column.
                let i = partial_sql_query_result.columns.iter().position(|k| k == column)?;
                // Return a tuple of the SQL column name and the SqlValue at the given index.
                Some((column.clone(), values.get(i)?.clone()))
            }).filter_map(|pair| pair).collect();
            collection
        }).collect();

        Ok(collection)
    }
}