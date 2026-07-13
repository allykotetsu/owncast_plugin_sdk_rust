use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ActionButton {
    pub(crate) title: String,
    pub(crate) url: Option<String>,
    pub(crate) html: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) color: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) open_externally: Option<String>
}