use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatMessage {
    pub(crate) id: String,
    pub(crate) user: Option<User>,
    pub(crate) client_id: Option<u64>,
    pub(crate) body: String,
    pub(crate) timestamp: String
}