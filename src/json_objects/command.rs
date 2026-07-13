use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Command {
    pub(crate) name: String,
    pub(crate) prefix: String,
    pub(crate) description: Option<String>,
    pub(crate) usage: Option<String>,
    pub(crate) aliases: Option<Vec<String>>,
    pub(crate) mod_only: Option<bool>,
    pub(crate) case_sensitive: Option<bool>,
    pub(crate) cooldown_ms: Option<u128>
}