use serde::Deserialize;
use crate::json_objects::user::User;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatMessage {
    id: String,
    user: Option<User>,
    client_id: Option<u32>,
    body: String,
    timestamp: String
}