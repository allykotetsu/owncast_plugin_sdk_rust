use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SSEConnectionEvent {
    pub(crate) channel: String,
    pub(crate) connection_id: u64,
    pub(crate) user: Option<User>
}