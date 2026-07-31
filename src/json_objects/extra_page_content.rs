use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExtraPageContent {
    pub(crate) slug: String,
    pub(crate) content: Option<String>,
}