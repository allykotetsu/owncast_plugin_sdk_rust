use extism_pdk::{ToBytes, FromBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromBytes, ToBytes, Clone, Debug)]
#[serde(untagged)]
#[encoding(Json)]
pub enum SqlValue {
    Null(()),
    Boolean(bool),
    Number(i64),
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
        Self::Number(i)
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