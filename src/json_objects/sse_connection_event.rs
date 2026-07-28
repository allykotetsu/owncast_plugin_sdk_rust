use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SseConnectionEvent {
    pub channel: String,
    pub connection_id: u64,
    pub user: Option<User>
}