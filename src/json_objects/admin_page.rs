use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AdminPage {
    pub(crate) title: String,
    pub(crate) path: String,
    pub(crate) icon: Option<String>,
}