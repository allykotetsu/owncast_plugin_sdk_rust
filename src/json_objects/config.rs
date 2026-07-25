use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub(crate) enum Config {
    String { default: String, description: String },
    Number { default: i16, description: String },
    Boolean { default: bool, description: String }
}