use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub user: Option<User>,
    pub client_id: Option<u64>,
    pub body: String,
    pub timestamp: String
}