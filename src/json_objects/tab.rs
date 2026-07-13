use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tab {
    pub(crate) title: String,
    pub(crate) slug: String,
    pub(crate) content: Option<String>,
}