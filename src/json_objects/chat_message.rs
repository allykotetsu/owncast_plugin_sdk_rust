use extism_pdk::{FromBytes, Json};
use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

#[derive(Serialize, Deserialize, Clone, FromBytes)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatMessage {
    pub id: String,
    pub user: Option<User>,
    pub client_id: Option<i64>,
    pub body: String,
    pub timestamp: String
}