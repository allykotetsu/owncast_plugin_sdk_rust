use extism_pdk::{FromBytes, Json};
use serde::{Deserialize, Serialize};
use crate::json_objects::user::User;

#[derive(Serialize, Deserialize, Clone, FromBytes, Debug)]
#[serde(rename_all = "camelCase")]
#[encoding(Json)]
pub struct ChatMessage {
    pub id: String,
    pub user: Option<User>,
    pub client_id: Option<i64>,
    pub body: String,
    pub timestamp: String
}

impl TryFrom<&ChatMessage> for i64 {
    type Error = ();

    fn try_from(ChatMessage { client_id, .. } : &ChatMessage) -> Result<Self, Self::Error> {
        match client_id {
            None => Err(()),
            Some(client_id) => Ok(*client_id)
        }
    }
}