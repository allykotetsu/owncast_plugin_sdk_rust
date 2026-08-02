use extism_pdk::{FromBytes, Json};
use serde::Deserialize;

#[derive(Deserialize, FromBytes, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatClient {
    pub id: u64,
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub connected_at: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub message_count: i64,
}