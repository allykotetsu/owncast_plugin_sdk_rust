use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatClient {
    pub(crate) id: u64,
    pub(crate) user_id: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) connected_at: Option<String>,
    pub(crate) user_agent: Option<String>,
    pub(crate) ip_address: Option<String>,
    pub(crate) message_count: u64,
}