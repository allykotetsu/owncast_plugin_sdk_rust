use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SseConnectionEvent {
    pub channel: String,
    pub connection_id: u64,
    pub user: Option<User>
}