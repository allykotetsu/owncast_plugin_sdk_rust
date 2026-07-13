use serde::{Deserialize, Serialize};

// TODO remove
#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum ConfigType {
    String(String),
    Number(i64),
    Boolean(bool)
}