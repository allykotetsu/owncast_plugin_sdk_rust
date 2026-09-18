use serde::Deserialize;

/// An object for when retrieving COUNT(*) data on a SQL SELECT query.
#[derive(Deserialize)]
pub struct Count {
    /// How many rows were [`Count`]ed.
    #[serde(rename = "COUNT(*)")]
    pub count: i64
}