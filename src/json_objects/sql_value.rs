use extism_pdk::{ToBytes, FromBytes, Json};
use serde::{Deserialize, Serialize};

/// An object representing a SQL value.
#[derive(Serialize, Deserialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(untagged)]
#[encoding(Json)]
pub enum SqlValue {
    /// A null [`SqlValue`].
    Null(()),

    /// A boolean [`SqlValue`].
    Boolean(bool),

    /// An integer [`SqlValue`].
    Integer(i64),

    /// A float [`SqlValue`].
    Float(f64),

    /// A string [`SqlValue`].
    String(String)
}

impl From<()> for SqlValue {
    fn from(_: ()) -> Self {
        Self::Null(())
    }
}

impl From<bool> for SqlValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<i64> for SqlValue {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<f64> for SqlValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<&str> for SqlValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<String> for SqlValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<SqlValue> for Vec<SqlValue> {
    fn from(sql_value: SqlValue) -> Self {
        vec![sql_value]
    }
}