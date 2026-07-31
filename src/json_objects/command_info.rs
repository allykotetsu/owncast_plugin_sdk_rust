use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommandInfo {
    pub(crate) name: String,
    pub(crate) prefix: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) usage: Option<String>,
    pub(crate) aliases: Option<Vec<String>>,
    pub(crate) mod_only: Option<bool>,
    pub(crate) case_sensitive: Option<bool>,
    pub(crate) cooldown_ms: Option<i64>
}