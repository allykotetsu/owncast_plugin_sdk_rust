use serde::{Deserialize, Serialize};
use crate::json_objects::config_type::ConfigType;

// TODO custom de/serialize logic

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) r#type: ConfigType,
    pub(crate) description: String
}
