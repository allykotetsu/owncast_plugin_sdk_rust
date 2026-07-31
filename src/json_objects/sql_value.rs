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